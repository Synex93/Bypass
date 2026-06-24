// -- Add up migration script here
// CREATE TABLE IF NOT EXISTS users (
//     id BIGSERIAL PRIMARY KEY,
//     name TEXT NOT NULL,
//     email TEXT NOT NULL UNIQUE,
//     password TEXT NOT NULL,
//     created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
// );

// USERS TABLE STRUCT
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
