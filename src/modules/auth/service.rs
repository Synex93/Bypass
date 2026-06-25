use crate::{
    error::AppError,
    modules::auth::{
        dto::{InfoResp, LoginReq, LoginResp},
        repo,
    },
    utils::{jwt_helper as jwt, password_helper as helper},
};

pub async fn login(req: LoginReq) -> Result<LoginResp, AppError> {
    let user = repo::select_user_by_email(req.email.as_str())
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !helper::verify_password(&req.password, &user.password)? {
        return Err(AppError::Unauthorized);
    }

    Ok(LoginResp {
        token: jwt::create_jwt(user.id, &user.email)?,
    })
}

pub async fn info(id: i64) -> Result<InfoResp, AppError> {
    let user = repo::select_userinfo_by_id(id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(user)
}
