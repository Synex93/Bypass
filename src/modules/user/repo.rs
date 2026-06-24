use super::model::User;
use crate::{database, error::AppError};
use sqlx::PgPool;

pub fn gen_user_id() -> i64 {
    rand::random_range(100_000_000_000_000_000i64..=999_999_999_999_999_999i64)
}

pub async fn create(name: &str, email: &str, password: &str) -> Result<User, AppError> {
    let pool = database::get_pool()?;
    if exists_by_email(pool, email).await? {
        return Err(AppError::Conflict("email already exists".to_string()));
    }
    Ok(sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, password, created_at, updated_at
        "#,
        gen_user_id(),
        name,
        email,
        password
    )
    .fetch_one(pool)
    .await?)
}

pub async fn exists_by_email(pool: &PgPool, email: &str) -> Result<bool, AppError> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM users WHERE email = $1
        )
        "#,
        email
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(false);

    Ok(exists)
}
