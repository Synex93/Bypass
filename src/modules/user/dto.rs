use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[validate(email(message = "invalid email"))]
    pub email: String,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResp {
    pub id: i64,
    pub name: String,
    pub email: String,
}
