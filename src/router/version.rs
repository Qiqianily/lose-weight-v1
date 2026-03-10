use std::net::SocketAddr;

use axum::{
    debug_handler,
    extract::{ConnectInfo, State},
};

use crate::{
    response::{ApiResult, resp::ApiResponse},
    state::app_state::AppState,
};

// Define a route for getting the current version of the API
pub fn get_version_router() -> axum::Router<AppState> {
    axum::Router::new().route("/version", axum::routing::get(get_current_version_handler))
}

#[tracing::instrument(
    name = "Query current version",
    skip_all,
    fields(get_version = "current version",ip = %addr.ip())
)]
#[debug_handler]
pub async fn get_current_version_handler(
    State(AppState { inner, .. }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> ApiResult<ApiResponse<String>> {
    tracing::info!("Querying current version ...");
    tracing::info!("IP地址为 {} 正在获取当前的版本信息...", addr.ip());
    let version = format!("v{}", inner.version);
    // return the current version
    Ok(ApiResponse::success(version))
}
