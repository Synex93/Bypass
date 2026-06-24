use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    Conflict(String),
    Internal(anyhow::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.code().as_deref() == Some("23505")
                && db_error.constraint() == Some("users_email_key")
            {
                return AppError::Conflict("email already exists".to_string());
            }
        }

        AppError::Internal(error.into())
    }
}
impl From<argon2::password_hash::Error> for AppError {
    fn from(error: argon2::password_hash::Error) -> Self {
        AppError::Internal(anyhow::anyhow!("password hash failed: {}", error))
    }
}
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", message),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "authentication required".to_string(),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "permission denied".to_string(),
            ),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, "NOT_FOUND", message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, "CONFLICT", message),
            AppError::Internal(error) => {
                tracing::error!(error = %error, "internal server error");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    "internal server error".to_string(),
                )
            }
        };

        let body = ErrorResponse {
            error: ErrorBody { code, message },
        };

        (status, Json(body)).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Internal(error)
    }
}
