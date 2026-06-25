use crate::{
    middleware::auth as auth_middleware,
    modules::{auth, user},
};
use axum::{Router, middleware, routing};
use tower_http::trace::TraceLayer;

pub fn get() -> Router {
    Router::new()
        .route("/", routing::get(async || "hello Bypass!"))
        .nest("/user", user::router())
        .nest("/auth", auth::router())
        .layer(middleware::from_fn(auth_middleware))
        .layer(TraceLayer::new_for_http())
}
