use serde::{Deserialize, Serialize};

use super::user::GithubUserInfo;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: Option<i32>,
    pub user: Option<GithubUserInfo>,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
    pub article: Option<u32>,
}