pub mod errors;
pub mod resp;

/// define response api result type
pub type ApiResult<T> = anyhow::Result<T, errors::ApiError>;
