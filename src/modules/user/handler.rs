use super::dto::{RegisterReq, RegisterResp};
use super::service;
use crate::error::AppError;
use axum::{Json, Router, http::StatusCode, routing};
use validator::Validate;

pub fn router() -> Router {
    Router::new().route("/register", routing::post(register))
}

pub async fn register(
    Json(req): Json<RegisterReq>,
) -> Result<(StatusCode, Json<RegisterResp>), AppError> {
    req.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok((StatusCode::CREATED, Json(service::create(req).await?)))
}
