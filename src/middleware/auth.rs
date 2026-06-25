use axum::{
    extract::Request,
    http::{Method, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::{error::AppError, utils::jwt_helper};

pub async fn auth(mut req: Request, next: Next) -> Result<Response, AppError> {
    if is_public_api(req.method(), req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let token_data = jwt_helper::verify_jwt(token).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

fn is_public_api(method: &Method, path: &str) -> bool {
    (method == Method::GET && path == "/")
        || (method == Method::POST && path == "/auth/login")
        || (method == Method::POST && path == "/user/register")
        || (method == Method::POST && path == "/user/check_email")
}
