use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
