use std::sync::Arc;

use ntex::web::{
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{errors::CustomError, models::{article::Article, user::Admin}, AppState};

// #[web::post("/article")]
pub async fn new_article(
    _: Admin,
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let db_pool = &state.db_pool; // 会修改内容

    sqlx::query!(
        "INSERT INTO articles (title, content) VALUES ($1, $2)",
        article.title,
        article.content,
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body("新增文章成功！"))
}
