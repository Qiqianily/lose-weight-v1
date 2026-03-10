use crate::{
    response::{ApiResult, errors::ApiError},
    state::app_state::AppState,
};

pub mod version;

/// combine all the routes into one router
pub fn merge_router() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/get/current", version::get_version_router())
        .fallback(async || -> ApiResult<()> {
            // 路径找不到
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        }) // 方法不允许
}
