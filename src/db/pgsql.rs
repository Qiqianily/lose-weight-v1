use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::conf::database::DbConfig;

/// 查询数据库连接状态
#[derive(sqlx::FromRow)]
struct ConnectionStats {
    total_connections: i64,
    active_connections: i64,
    idle_connections: i64,
}

/// 使用配置初始化数据库
pub async fn init_database_pool_with_config(config: &DbConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .min_connections(config.min_connections())
        .max_connections(config.max_connections())
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs()))
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs()))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs()))
        .max_lifetime(Duration::from_secs(config.max_lifetime_secs()))
        // .sqlx_logging(config.sqlx_logging())
        // .set_schema_search_path(config.schema())
        .connect(config.url())
        .await
        .map_err(|e| anyhow::anyhow!("无法连接到数据库：{}", e))?;

    // 测试连接
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("测试连接数据库失败：{}", e))?;

    // 记录数据库信息
    log_detailed_database_info(&pool).await?;

    tracing::info!(
        "✅ Database pool initialized with {} min, {} max connections",
        config.min_connections(),
        config.max_connections()
    );

    Ok(pool)
}

/// 记录详细的数据库信息
async fn log_detailed_database_info(pool: &PgPool) -> anyhow::Result<()> {
    // 获取版本信息
    let version: String = sqlx::query_scalar("SELECT version()")
        .fetch_one(pool)
        .await?;

    tracing::info!("Database version: {}", version.trim());

    // 获取当前连接数
    let stats: ConnectionStats = sqlx::query_as(
        r#"
            SELECT
                COUNT(*) AS total_connections,
                COUNT(*) FILTER (WHERE state = 'active') AS active_connections,
                COUNT(*) FILTER (WHERE state = 'idle') AS idle_connections
            FROM pg_stat_activity
            WHERE datname = current_database()
            "#,
    )
    .fetch_one(pool)
    .await?;

    tracing::info!(
        "Current connections - Total: {}, Active: {}, Idle: {}",
        stats.total_connections,
        stats.active_connections,
        stats.idle_connections
    );

    Ok(())
}
