use crate::{
    error::AppError,
    modules::user::dto::{RegisterReq, RegisterResp},
};

use super::repo;

// create user
pub async fn create(req: RegisterReq) -> Result<RegisterResp, AppError> {
    let user = repo::create(req.name.as_str(), req.email.as_str(), req.password.as_str()).await?;
    Ok(RegisterResp {
        id: user.id,
        name: user.name,
        email: user.email,
    })
}
