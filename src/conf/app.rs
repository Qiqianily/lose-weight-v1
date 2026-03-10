use crate::conf::base::BaseConfig;
use crate::conf::database::DbConfig;
use crate::conf::redis::RedisConfig;
use anyhow::Context;
use clap::Parser;
use config::{Config, Environment, File, FileFormat};
use std::env;

/// 命令行输入配置文件路径
#[derive(Parser)]
#[clap(
    name = "lose_weight_server",
    version = "1.0",
    author = "Qiqianily",
    about = "lose weight backend project"
)]
pub struct CmdOpts {
    #[clap(long)]
    pub config: Option<String>,
}

// Define the application Config struct
#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    version: String,
    base: BaseConfig,
    database: DbConfig,
    redis: RedisConfig,
    is_dev: bool,
}
impl AppConfig {
    // load the config file
    pub fn load() -> anyhow::Result<Self> {
        // 根据环境变量来确定是开环境式还是生产环境
        let run_mode = env::var("APP_ENV").unwrap_or_else(|_| "prod".into());
        // 解析命令行参数
        let cmd_config = CmdOpts::parse();
        let file_path = Self::resolve_config_path(&cmd_config, &run_mode);
        println!("Using config file: {file_path}");
        let config_builder = Config::builder()
            .add_source(
                File::with_name(&file_path)
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            );
        config_builder
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config file: {file_path}"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config into AppConfig"))
    }
    /// 根据运行时的模式加载文件路径
    fn resolve_config_path(cmd_opts: &CmdOpts, run_mode: &str) -> String {
        match &cmd_opts.config {
            Some(path) => path.clone(),
            None => {
                if run_mode == "dev" {
                    "./dev".into()
                } else {
                    "/app/conf/prod.toml".into()
                }
            }
        }
    }
    pub fn get_current_version(&self) -> &str {
        &self.version
    }
    pub fn base(&self) -> &BaseConfig {
        &self.base
    }
    pub fn database(&self) -> &DbConfig {
        &self.database
    }
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }
    pub fn is_dev(&self) -> bool {
        self.is_dev
    }
}
