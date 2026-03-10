/// 服务端口和日志信息相关配置
#[derive(Debug, serde::Deserialize)]
pub struct BaseConfig {
    host: String,
    port: u16,
    log_level: String,
    allowed_hosts: Option<Vec<String>>,
}
impl BaseConfig {
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    // get allowed hosts from the profile if none use the default.
    pub fn allowed_host(&self) -> Vec<&str> {
        self.allowed_hosts
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_else(|| vec!["localhost", "127.0.0.1"])
    }
}
