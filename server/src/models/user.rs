use serde::Deserialize;

/// 解析前端 Github 授权登陆后传上来的 code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubUserInfo {
    /// Github 用户 ID
    pub id: i32,
    /// 用户名（不是昵称）
    pub login: String,
    /// 用户头像
    pub avatar_url: String,
}