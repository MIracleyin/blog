use ntex::web::{
    self,
    types::{Json, State},
};
use std::sync::Arc;

use crate::{errors::CustomError, models::article::Article, AppState};

#[web::get("/articles")]
pub async fn get_all_articles(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Article>>, CustomError> {
    let db_pool = &state.db_pool;
    let articles = sqlx::query!("SELECT * FROM articles")
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| Article {
            id: Some(i.id as u32),
            title: i.title.clone(),
            content: i.content.clone(),
            date: Some(i.date), // 为什么修改数据库后，这里的代码就不报错了呢？
        })
        .collect::<Vec<Article>>();

    Ok(Json(articles))
}
