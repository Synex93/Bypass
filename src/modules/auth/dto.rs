use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::auth::model::UserInfo;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(email(message = "invalid email"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
}
pub type InfoResp = UserInfo;
