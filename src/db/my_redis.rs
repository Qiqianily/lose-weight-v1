use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis::{self, AsyncCommands, RedisResult};
// 类型别名简化
type RedisPool = Pool<RedisConnectionManager>;

/// 创建 Redis 连接池
///
/// # 参数
/// - url: Redis 连接字符串 (例如: "redis://127.0.0.1:6379")
/// - max_pool_size: 连接池最大大小
/// - max_idle: 连接池最大空闲数
/// - timeout_sec: 连接超时时间(秒)
pub async fn create_redis_pool(
    url: &str,
    max_pool_size: u64,
    max_idle: u64,
    timeout_sec: u64,
) -> RedisResult<RedisPool> {
    let client = redis::Client::open(url)?;
    let manager = RedisConnectionManager::new(client);

    Ok(Pool::builder()
        .max_open(max_pool_size)
        .max_idle(max_idle)
        .max_lifetime(Some(std::time::Duration::from_secs(3600 * 8))) // 最大连接时间为 8 个小时
        .get_timeout(Some(std::time::Duration::from_secs(timeout_sec)))
        .max_idle_lifetime(Some(std::time::Duration::from_secs(3600))) // 空闲连接时间为 1 个小时
        .health_check_interval(Some(std::time::Duration::from_secs(60))) // 每分钟健康检查
        .build(manager))
}

/// Redis 操作客户端
#[derive(Clone, Debug)]
pub struct RedisClient {
    redis_pool: RedisPool,
}

impl RedisClient {
    pub fn new(redis_pool: RedisPool) -> Self {
        Self { redis_pool }
    }
    /// 获取连接（内部使用）
    async fn get_conn(&self) -> RedisResult<mobc::Connection<RedisConnectionManager>> {
        self.redis_pool.get().await.map_err(|err| {
            redis::RedisError::from((redis::ErrorKind::IoError, "连接池获取失败", err.to_string()))
        })
    }
    /// 设置键值
    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut conn = self.get_conn().await?;
        conn.set(key, value).await
    }

    /// 设置带过期时间的键值
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> RedisResult<()> {
        let mut conn = self.get_conn().await?;
        conn.set_ex(key, value, seconds).await
    }

    /// 获取键值
    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let mut conn = self.get_conn().await?;
        conn.get(key).await
    }

    /// 设置过期时间（返回是否设置成功）为已存在的 Key 设置过期时间
    pub async fn expire(&self, key: &str, seconds: i64) -> RedisResult<bool> {
        let mut conn = self.get_conn().await?;
        // EXPIRE 返回 1 表示成功设置，0 表示键不存在
        let result: i64 = conn.expire(key, seconds).await?;
        Ok(result == 1)
    }

    /// 删除键（返回删除的键数量）
    pub async fn del(&self, key: &str) -> RedisResult<usize> {
        let mut conn = self.get_conn().await?;
        conn.del(key).await
    }

    /// 检查键是否存在，返回 boolean
    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.get_conn().await?;
        conn.exists(key).await
    }

    /// 自增操作，步长为 1
    pub async fn incr(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_conn().await?;
        conn.incr(key, 1).await
    }

    /// 设置键值并返回旧值
    pub async fn get_set(&self, key: &str, value: &str) -> RedisResult<Option<String>> {
        let mut conn = self.get_conn().await?;
        conn.getset(key, value).await
    }

    /// 检查并设置键值（原子操作）仅在不存在的情况下设置
    pub async fn set_nx(&self, key: &str, value: &str) -> RedisResult<bool> {
        let mut conn = self.get_conn().await?;
        conn.set_nx(key, value).await
    }

    /// 获取键的剩余生存时间（秒）
    pub async fn ttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_conn().await?;
        conn.ttl(key).await
    }
}
