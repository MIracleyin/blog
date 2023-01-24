use ntex::web::{
    self,
    types::{Json, State},
};
use std::sync::{Arc, Mutex};

use crate::{errors::CustomError, models::article::Article, AppState};

#[web::get("/articles")]
pub async fn get_articles(
    state: State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Article>>, CustomError> {
    let db_pool = &state.lock().unwrap().db_pool;
    let articles = sqlx::query!("SELECT * FROM articles")
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| Article {
            id: i.id as u32,
            title: i.title.clone(),
            content: i.content.clone(),
            date: i.date,
        })
        .collect::<Vec<Article>>();

    Ok(Json(articles))
}
