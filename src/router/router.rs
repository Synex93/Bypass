use crate::modules::user;
use axum::{Router, routing};
use tower_http::trace::TraceLayer;

pub fn get() -> Router {
    Router::new()
        .route("/", routing::get(async || "hello Bypass!"))
        .nest("/user", user::router())
        .layer(TraceLayer::new_for_http())
}
