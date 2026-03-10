/// 自定义 ApiResponse 封装
///
/// # 成员
/// - code：状态码
/// - message：返回的信息
/// - data：返回的数据，可选。
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")] // 如果是空的就跳过序列化
    pub data: Option<T>,
}
impl<T: serde::Serialize> ApiResponse<T> {
    /// 构造函数
    pub fn new(code: i32, message: String, data: Option<T>) -> ApiResponse<T> {
        Self {
            code,
            message,
            data,
        }
    }
    /// 成功返回，自定义消息和数据 200,custom msg,Data
    pub fn ok<M: AsRef<str>>(message: M, data: Option<T>) -> Self {
        Self::new(200, String::from(message.as_ref()), data)
    }

    /// 成功返回，默认消息和数据 200,success,ata
    pub fn success(data: T) -> Self {
        Self::new(200, "success".to_string(), Some(data))
    }

    /// 没有任何数据的成功返回 200,success,None
    pub fn success_no_data() -> Self {
        Self::new(200, "success".to_string(), None)
    }

    /// 没有任何数据的成功返回 200,Msg,None
    pub fn success_with_msg<M: AsRef<str>>(message: M) -> Self {
        Self::new(200, String::from(message.as_ref()), None)
    }

    /// 错误返回，使用默认错误码-1和自定义消息 -1,custom msg,None
    pub fn err<M: AsRef<str>>(message: M) -> Self {
        Self::new(-1, String::from(message.as_ref()), None)
    }

    /// 错误返回，自定义错误码和消息 custom code,custom msg,None
    pub fn err_with_code<M: AsRef<str>>(code: i32, message: M) -> Self {
        Self::new(code, String::from(message.as_ref()), None)
    }
}

/// 为 ApiResponse 实现 IntoResponse
impl<T: serde::Serialize> axum::response::IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
