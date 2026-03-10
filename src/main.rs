use lose_weight::{app, conf, log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 读取配置信息
    let config = conf::get_app_config();

    // 2. 初始化日志，为了防止多线程日志写入不完整，要保留 guard，main 函数结束时释放
    let _guard = log::logger::init_logger_with_file(config.base().log_level()).await?;
    // 3. 启动服务
    app::server::Server::new(config).start_server().await?;

    Ok(())
}
