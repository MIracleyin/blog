use std::sync::Arc;

use ntex::web::types::{Json, Path, State};

use crate::{
    article,
    errors::CustomError,
    models::{comment::{Comment, self}, user::GithubUserInfo},
    AppState,
};

pub async fn get_comments_for_article(
    article_id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Comment>>, CustomError> {
    let db_pool = &state.db_pool;

    let article_id = article_id.0;

    // 从文章id中查找所有评论，拿到 user_id content date，并从 users 表里拿到相同的 user_id（对应 users 表里的 id） 记录的 name, avatar_url
    let comments = sqlx::query!(
        "SELECT comments.id, comments.user_id, comments.content, comments.date, users.name, users.avatar_url FROM comments JOIN users ON comments.user_id = users.id WHERE comments.article = $1", article_id as i32
    ).fetch_all(db_pool).await?.iter().map(|i| Comment {
        id: i.id,
        user: Some(GithubUserInfo {
            id: i.user_id,
            login: i.name.clone(),
            avatar_url: i.avatar_url.clone(),
        }),
        content: i.content.clone(),
        date: Some(i.date),
        article: None,
    })
    .collect::<Vec<Comment>>();

    Ok(Json(comments))
}

