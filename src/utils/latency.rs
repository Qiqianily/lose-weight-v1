use std::time::Duration;

use axum::http::Response;
use tower_http::trace::OnResponse;
use tracing::Span;

/// Latency on response 响应延时
#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
            latency = %Latency(latency),
            status = response.status().as_u16(),
            "Finished processing request"
        )
    }
}

/// Latency 延迟用时
struct Latency(Duration);

/// 重写了 display trait 如果是 ms 就返回，否则返回 μs
impl std::fmt::Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} ms", self.0.as_millis())
        } else {
            write!(f, "{} μs", self.0.as_micros())
        }
    }
}
