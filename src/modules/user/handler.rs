use super::dto::{RegisterReq, RegisterResp};
use super::service;
use crate::error::AppError;
use crate::modules::user::dto::{CheckEmailReq, CheckEmailResp};
use axum::{Json, Router, http::StatusCode, routing};
use validator::Validate;

pub fn router() -> Router {
    Router::new()
        .route("/register", routing::post(register))
        .route("/check_email", routing::post(check_email))
}

pub async fn register(
    Json(req): Json<RegisterReq>,
) -> Result<(StatusCode, Json<RegisterResp>), AppError> {
    req.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok((StatusCode::CREATED, Json(service::create(req).await?)))
}

pub async fn check_email(
    Json(req): Json<CheckEmailReq>,
) -> Result<(StatusCode, Json<CheckEmailResp>), AppError> {
    req.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok((StatusCode::OK, Json(service::check_email(req).await?)))
}
