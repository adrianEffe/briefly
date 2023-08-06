use axum::http;
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestSpan;

impl<B> tower_http::trace::MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        tracing::error_span!(
            "rq",
            id = %Uuid::new_v4().to_string(),
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
        )
    }
}
