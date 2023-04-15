use axum::body::HttpBody;
use axum::routing::get;
use axum::Router;

pub fn get_app<B, S>() -> Router<S, B>
where
    B: Send + HttpBody + 'static,
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
