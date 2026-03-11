use crate::middlewares::auth::jwt::{JWT, get_default_jwt};
use crate::response::errors::ApiError;
use axum::http::{Request, Response};
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JwtAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::new(JwtAuth::new(get_default_jwt())));

/// JwtAuth struct
#[derive(Clone)]
pub struct JwtAuth {
    jwt: &'static JWT,
}
/// JwtAuth constructor
impl JwtAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

/// impl authorize request for JwtAuth
impl AsyncAuthorizeRequest<axum::body::Body> for JwtAuth {
    type RequestBody = axum::body::Body;
    type ResponseBody = axum::body::Body;

    type Future = std::pin::Pin<
        Box<
            dyn Future<Output = Result<Request<Self::ResponseBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: Request<axum::body::Body>) -> Self::Future {
        let jwt = self.jwt;
        Box::pin(async move {
            // get the token from request header
            let token = request
                .headers()
                .get(axum::http::header::AUTHORIZATION)
                .map(|value| -> Result<&str, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization 请求头不是一个有效的字符串",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization 请求头必须以 Bearer 开头!",
                            ))
                        })?;
                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(String::from("请求头中没有 Authorization 字段"))
                })?;
            let principal = jwt
                .decode(token)
                .map_err(|err| ApiError::Unauthenticated(format!("没有登陆或登陆已过期 {err}")))?;
            request.extensions_mut().insert(principal);
            Ok(request)
        })
    }
}

/// public method to get the static AUTH_LAYER pointer
pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JwtAuth> {
    &AUTH_LAYER
}
