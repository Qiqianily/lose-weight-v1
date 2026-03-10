use axum::response::IntoResponse;

use crate::response::resp::ApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Biz(String),
    #[error("没找到！")]
    NotFound,
    #[error("方法不支持！")]
    MethodNotAllowed,
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("服务端错误！")]
    InternalServerError,
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::Biz(_) => axum::http::StatusCode::OK,
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::InternalServerError | ApiError::DatabaseError(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

/// 将错误转换为响应
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code(),
            axum::Json(ApiResponse::<()>::err(self.to_string())),
        )
            .into_response()
    }
}

/// 把错误转换成返回值
impl From<ApiError> for axum::http::Response<axum::body::Body> {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}
