use std::{future::Future, pin::Pin, sync::Arc};

use cookie::Cookie;
use ntex::{
    http::HttpMessage,
    web::{ErrorRenderer, FromRequest},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{errors::CustomError, AppState};

/// 解析前端 Github 授权登陆后传上来的 code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubUserInfo {
    /// Github 用户 ID
    pub id: i32,
    /// 用户名（不是昵称）
    pub login: String,
    /// 用户头像
    pub avatar_url: String,
}

/// 网站中的所有用户
#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    // pub access_token: String,
}

/// 网站中的管理员
#[derive(Debug, Clone)]
pub struct Admin {
    pub id: i32,
    // pub access_token: String,
}

impl<E: ErrorRenderer> FromRequest<E> for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_none()
            {
                // 查询无记录
                // 要求登录
                return Err(CustomError::AuthFailed(
                    "还未在本站使用 Github 登录过，请登录".into(),
                ));
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fut)
    }
}

impl<E: ErrorRenderer> FromRequest<E> for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => {
                    return Err(e);
                }
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_some()
            {
                // 用户表中查询到了
                // 需要管理员权限
                // 管理员 GitHub ID
                if user_id != 29721742 {
                    // 用户不是管理员
                    return Err(CustomError::AuthFailed(
                        "你不是管理员，无权执行该操作".into(),
                    ));
                }
            } else {
                // 未查到
                return Err(CustomError::AuthFailed(
                    "还未在本站使用 Github 登录过，请登录".into(),
                ));
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fut)
    }
}

async fn get_user_id(access_token: &Cookie<'_>) -> Result<i32, CustomError> {
    let client = Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        .header("User-Agent", "miracleyin")
        .send()
        .await;

    let user_id = match user_info {
        Ok(r) => match r.json::<GithubUserInfo>().await {
            Ok(i) => i.id,
            // 无法解析
            Err(_) => {
                return Err(CustomError::BadRequest(
                    "无法获取 Github 用户信息，可能是提供了不正确的 access token，请重新登录"
                        .into(),
                ))
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取 Github 用户信息，请重试".into(),
            ))
        }
    };
    Ok(user_id)
}
