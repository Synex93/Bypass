use super::repo;
use crate::{
    error::AppError,
    modules::user::dto::{CheckEmailReq, CheckEmailResp, RegisterReq, RegisterResp},
    utils::password_helper as helper,
};

// create user
pub async fn create(req: RegisterReq) -> Result<RegisterResp, AppError> {
    let pass = helper::hash_password(&req.password)?;
    let user = repo::create(&req.name, &req.email, &pass).await?;
    Ok(RegisterResp {
        id: user.id,
        name: user.name,
        email: user.email,
    })
}

// check email
pub async fn check_email(req: CheckEmailReq) -> Result<CheckEmailResp, AppError> {
    Ok(CheckEmailResp {
        is_exist: repo::exists_by_email(&req.email).await?,
    })
}
