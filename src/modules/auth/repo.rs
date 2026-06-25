use crate::{database, error::AppError, modules::auth::model::UserInfo};

use super::model::User;

pub async fn select_user_by_email(email: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(database::get_pool()?)
    .await?;

    Ok(user)
}

pub async fn select_userinfo_by_id(id: i64) -> Result<Option<UserInfo>, AppError> {
    let userinfo = sqlx::query_as!(
        UserInfo,
        r#"
        SELECT id, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(database::get_pool()?)
    .await?;

    Ok(userinfo)
}
