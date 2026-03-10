use std::sync::LazyLock;

use crate::conf::app::AppConfig;

pub mod app;
pub mod base;
pub mod database;
pub mod redis;

// set the static config
static APP_CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initial app config"));

// get the static config pointer
pub fn get_app_config() -> &'static AppConfig {
    &APP_CONFIG
}
