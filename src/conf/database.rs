/// 数据库连接池相关配置
#[derive(Debug, serde::Deserialize)]
pub struct DbConfig {
    url: String,
    min_connections: u32,
    max_connections: u32,
    connect_timeout_secs: u64,
    acquire_timeout_secs: u64,
    idle_timeout_secs: u64,
    max_lifetime_secs: u64,
    sqlx_logging: bool,
    schema: String,
    timezone: String,
}
impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: Default::default(),
            min_connections: 3,
            max_connections: 10,
            connect_timeout_secs: 10,
            acquire_timeout_secs: 6,
            idle_timeout_secs: 600,
            max_lifetime_secs: 1800,
            sqlx_logging: false,
            schema: "public".into(),
            timezone: "Asia/Shanghai".into(),
        }
    }
}

impl DbConfig {
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }
    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }
    pub fn connect_timeout_secs(&self) -> u64 {
        self.connect_timeout_secs
    }
    pub fn acquire_timeout_secs(&self) -> u64 {
        self.acquire_timeout_secs
    }
    pub fn idle_timeout_secs(&self) -> u64 {
        self.idle_timeout_secs
    }
    pub fn max_lifetime_secs(&self) -> u64 {
        self.max_lifetime_secs
    }
    pub fn sqlx_logging(&self) -> bool {
        self.sqlx_logging
    }
    pub fn schema(&self) -> &str {
        &self.schema
    }
    pub fn timezone(&self) -> &str {
        &self.timezone
    }
}
