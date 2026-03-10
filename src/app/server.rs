use std::net::SocketAddr;

use anyhow::Context;
use axum::{
    extract::{DefaultBodyLimit, Request},
    http::StatusCode,
};
use bytesize::ByteSize;
use tower_http::{normalize_path::NormalizePathLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{
    conf,
    db::{
        my_redis::{self, RedisClient},
        pgsql::init_database_pool_with_config,
        set_global_db, set_global_redis_client,
    },
    middlewares, router,
    state::app_state::AppState,
    utils::latency::LatencyOnResponse,
};

/// 服务端配置信息
///
/// - server_config: 一个静态的 app_config 配置
pub struct Server {
    pub server_config: &'static conf::app::AppConfig,
}

impl Server {
    /// 构造方法
    pub fn new(server_config: &'static conf::app::AppConfig) -> Self {
        Self { server_config }
    }
    /// 启动服务
    pub async fn start_server(&self) -> anyhow::Result<()> {
        // 1. 连接数据库 pgsql
        let db_pool = init_database_pool_with_config(self.server_config.database())
            .await
            .context("failed to init db pool")?;
        set_global_db(db_pool).await?;
        // 2. 连接 redis
        let redis_pool = my_redis::create_redis_pool(
            self.server_config.redis().url(),
            self.server_config.redis().max_open(),
            self.server_config.redis().max_idle(),
            self.server_config.redis().timeout_sec(),
        )
        .await
        .context("failed to set redis pool")?;
        // 创建 redis client
        let redis_client = RedisClient::new(redis_pool);
        set_global_redis_client(redis_client).await?;

        // new app state 创建 app 数据状态对象
        let app_state = AppState::new(self.server_config.get_current_version().to_string()).await;

        // create our application router 创建路由
        let app_router = self.build_router(app_state).await;
        // use axum to serve our application, listening on the specified address
        // 构建 http address
        let addr = format!(
            "{}:{}",
            self.server_config.base().host(),
            self.server_config.base().port()
        );
        // create listener 创建监听器
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        // info logs the address we're listening to on 输出日志。
        tracing::info!("🚀 listening on http://{}", listener.local_addr()?);
        // run our application on the listener
        // 运行服务
        axum::serve(
            listener,
            app_router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(shutdown_signal())
        .await?;
        // this point the application has stopped, so we can return
        tracing::info!("✅ server terminated gracefully");
        Ok(())
    }
    /// 构造路由，并添加各种中间件
    ///
    /// # 参数
    /// - state: app 的数据状态
    pub async fn build_router(&self, state: AppState) -> axum::Router {
        // time out 120 seconds 120 秒的超时
        let timeout = TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(120),
        );
        // request body size limit 10M 限制请求体的大小为 10M。
        let body_size_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);

        // cors layer setting 跨域中间件
        let cors_layer = middlewares::cors::app_cors();

        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request<axum::body::Body>| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);

        // trim trailing slash  /api/ ===> /api 去除后面的 “/”
        let normalize_path = NormalizePathLayer::trim_trailing_slash();

        // return the router 返回路由
        axum::Router::new()
            .nest("/api/v1", router::merge_router())
            .layer(timeout)
            .layer(body_size_limit)
            .layer(tracing)
            .layer(cors_layer)
            .layer(normalize_path)
            .with_state(state)
    }
}

// 处理打断信号，优雅关闭服务
// 中断信号处理，这个不能处理子任务中的耗时任务。
async fn shutdown_signal() {
    // 监听 Ctrl + c
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    // 监听 SIGTERM
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    // 跨平台代码兼容，Win 系统不支持 Unix 信号，加入这个是便于统一 select! 逻辑
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("received Ctrl+C signal, shutting down the server!");
        },
        _ = terminate => {
            tracing::info!("received Terminate signal, shutting down the server!");
        },
    }
}
