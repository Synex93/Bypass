use axum::Json;

use crate::builders::BUILDER_LANGUAGES;
use crate::builders::builders::BuilderLanguage;
use crate::error::AppError;

pub async fn builder_languages() -> Result<Json<Vec<BuilderLanguage>>, AppError> {
    Ok(Json(BUILDER_LANGUAGES.to_vec()))
}
