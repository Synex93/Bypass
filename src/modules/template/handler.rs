use axum::{Json, Router, routing};

use crate::{builders::builders::BuilderLanguage, error::AppError, modules::template::service};

pub fn router() -> Router {
    Router::new().route("/builder_languages", routing::get(builder_languages))
}

// get builder languages
pub async fn builder_languages() -> Result<Json<Vec<BuilderLanguage>>, AppError> {
    Ok(service::builder_languages().await?)
}
