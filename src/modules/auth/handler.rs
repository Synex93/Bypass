use super::dto::{LoginReq, LoginResp};
use super::service;
use crate::error::AppError;
use crate::modules::auth::dto::InfoResp;
use crate::utils::jwt_helper::Claims;
use axum::{Extension, Json, Router, http::StatusCode, routing};
use validator::Validate;

pub fn router() -> Router {
    Router::new()
        .route("/login", routing::post(login))
        .route("/info", routing::get(info))
}
pub async fn login(Json(req): Json<LoginReq>) -> Result<(StatusCode, Json<LoginResp>), AppError> {
    req.validate()
        .map_err(|err| AppError::BadRequest(err.to_string()))?;

    Ok((StatusCode::OK, Json(service::login(req).await?)))
}
pub async fn info(
    Extension(claims): Extension<Claims>,
) -> Result<(StatusCode, Json<InfoResp>), AppError> {
    Ok((StatusCode::OK, Json(service::info(claims.id).await?)))
}
