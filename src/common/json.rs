use axum::extract::FromRequest;
use axum_valid::HasValidate;

use crate::response::errors::ApiError;

/// 验证 Json 数据合法性
#[derive(Debug, Clone, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T> HasValidate for Json<T> {
    type Validate = T;

    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
