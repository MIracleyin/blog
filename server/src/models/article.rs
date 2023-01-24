use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Article {
    pub id: u32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub date: Option<chrono::NaiveDate>,
}