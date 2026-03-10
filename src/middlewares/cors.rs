use crate::conf;

/// app 跨域中间件
///
pub fn app_cors() -> tower_http::cors::CorsLayer {
    // 读取配置中的 hosts
    let allowed_hosts = conf::get_app_config().base().allowed_host();
    // 新建一个 vector 用来存放 host
    let mut origin_urls = Vec::new();
    // 遍历配置中的 host 加入到 origin urls 中
    for host in allowed_hosts.into_iter() {
        origin_urls.push(host.parse().unwrap());
    }
    // 创建跨域中间件
    tower_http::cors::CorsLayer::new()
        .allow_origin(origin_urls)
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_credentials(false) // cookies
        .max_age(std::time::Duration::from_secs(86400)) // set preflight request cache time one day
}
