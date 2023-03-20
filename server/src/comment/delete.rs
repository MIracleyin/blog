use std::sync::Arc;

use ntex::{web::types::{State, Path}};

use crate::{
    errors::CustomError,
    models::user::{Admin, User},
    AppState,
};

pub async fn delete_comment(
    user: User,
    admin: Option<Admin>,
    comment_id: Path<(u32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    let comment_id = comment_id.0;
    let user_id = user.id;

    let is_admim = admin.is_some();

    let rows_affected = if is_admim {
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id as i32)
            .execute(db_pool)
            .await?
    } else {
        sqlx::query!(
            "DELETE FROM comments WHERE id = $1 AND user_id = $2",
            comment_id as i32,
            user_id
        )
        .execute(db_pool)
        .await?
    }
    .rows_affected();

    if rows_affected == 0 {
        Err(CustomError::NotFound(
            "删除评论失败，可能是提供的评论 ID 不正确，或没有权限删除这条评论".into(),
        ))
    } else {
        Ok("删除评论成功！".into())
    }
}
