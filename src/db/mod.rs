use std::sync::OnceLock;

use sqlx::PgPool;

use crate::db::my_redis::RedisClient;

pub mod my_redis;
pub mod pgsql;

// 全局 Postgres 数据库连接池实例
static GLOBAL_DATABASE_POOL: OnceLock<PgPool> = OnceLock::new();
/// 全局 Redis 连接池实例
static GLOBAL_REDIS_CLIENT: OnceLock<RedisClient> = OnceLock::new();
/// 获取全局的静态 Postgres 数据库连接池引用
pub fn get_global_database_pool() -> &'static PgPool {
    GLOBAL_DATABASE_POOL.get().expect("database pool lost")
}
/// 初始化全局的静态数据库
pub async fn set_global_db(db: PgPool) -> anyhow::Result<()> {
    GLOBAL_DATABASE_POOL
        .set(db)
        .map_err(|_| anyhow::anyhow!("failed to set global database pool"))
}
pub async fn set_global_redis_client(client: RedisClient) -> anyhow::Result<()> {
    GLOBAL_REDIS_CLIENT
        .set(client)
        .map_err(|_| anyhow::anyhow!("failed to set global redis client"))
}
/// 获取全局的静态 Redis 连接池引用
pub fn get_global_redis_client() -> &'static RedisClient {
    GLOBAL_REDIS_CLIENT
        .get()
        .expect("global redis client not set")
}
