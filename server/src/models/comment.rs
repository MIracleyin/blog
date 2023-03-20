use serde::{Deserialize, Serialize};

use super::user::GithubUserInfo;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    // 评论 ID
    pub id: Option<i32>,
    // 展示评论用户的 Github 信息
    pub user: Option<GithubUserInfo>,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
    // 被评论的文章
    pub article: Option<u32>,
}