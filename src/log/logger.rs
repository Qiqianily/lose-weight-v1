use std::str::FromStr;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{Layer as _, Registry, layer::SubscriberExt, util::SubscriberInitExt};

use crate::utils::timezone::LocalTimer;

/// 初始化日志相关，将日志输出到文件中
///
/// # 参数
/// - logger_level: 日志等级
pub async fn init_logger_with_file(logger_level: &str) -> anyhow::Result<WorkerGuard> {
    // set logger level form params if there had error use default info level
    let level = tracing::level_filters::LevelFilter::from_str(logger_level)
        .unwrap_or(tracing::level_filters::LevelFilter::INFO);
    // create the log directory
    let log_dir = "logs";
    std::fs::create_dir_all(log_dir).expect("Failed to create log directory");
    // 按照当前日期生成日志文件名 weight_sys.log.2026-03-10
    let file_appender = rolling::daily(log_dir, "weight_sys.log");
    // 创建非阻塞写入器（后台启动一个专门用于写日志的线程）
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // 自定义日志输出文件的格式
    let file_layer = tracing_subscriber::fmt::Layer::default()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(level);

    // 自定义日志输出到控制台的格式
    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .with_timer(LocalTimer)
        .with_writer(std::io::stdout)
        .with_filter(level);

    Registry::default()
        .with(file_layer) // 写入文件
        .with(stdout_layer) // 输出到终端
        .init();
    Ok(guard)
}

/// 初始化日志相关，只输出到控制台
///
/// # 参数
/// - logger_level: 日志等级
pub async fn init_logger_without_file(logger_level: &str) -> anyhow::Result<()> {
    // set logger level form params if there had error use default info level
    let level = tracing::level_filters::LevelFilter::from_str(logger_level)
        .unwrap_or(tracing::level_filters::LevelFilter::INFO);

    // 自定义日志输出到控制台的格式
    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .with_timer(LocalTimer)
        .with_writer(std::io::stdout)
        .with_filter(level);

    Registry::default()
        .with(stdout_layer) // 输出到终端
        .init();
    Ok(())
}
