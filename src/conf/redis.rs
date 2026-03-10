/// redis 连接相关配置
#[derive(Debug, serde::Deserialize)]
pub struct RedisConfig {
    url: String,
    max_open: u64,
    max_idle: u64,
    timeout_sec: u64,
}

impl RedisConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn max_open(&self) -> u64 {
        self.max_open
    }
    pub fn max_idle(&self) -> u64 {
        self.max_idle
    }
    pub fn timeout_sec(&self) -> u64 {
        self.timeout_sec
    }
}
