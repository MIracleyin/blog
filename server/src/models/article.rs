use serde::{Deserialize, Serialize};

/// 文章详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
}

/// 文章预览
#[derive(Debug, Clone, Serialize)]
pub struct ArticlePreview {
    pub id: u32,
    pub title: String,
    pub date: chrono::NaiveDate,
}
