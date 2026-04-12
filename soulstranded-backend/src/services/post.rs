use sea_orm::DatabaseConnection;

use crate::{
    constants,
    database::{
        CommentInfo, NewComment, NewContent, PaginationData, PostInfo, add_comment, add_content,
        check_user_liked_post, delete_comment as db_delete_comment, delete_content, get_comments_by_post_id, get_post_pagination,
        like_post, unlike_post, update_comment as db_update_comment, update_content,
    },
    utils::verification::verify_login_token,
};

pub async fn publish_service(
    db: &DatabaseConnection,
    token: &str,
    content: String,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let new_post = NewContent {
        content,
        user_id: claims.id,
    };

    add_content(db, new_post).await.map_err(|e| {
        tracing::error!("Failed to add content to database: {}", e);
        constants::SYSTEM_ERROR
    })?;

    Ok(())
}

pub async fn get_posts_service(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
    filter_user_id: Option<i32>,
    current_user_id: Option<i32>,
) -> Result<PaginationData<PostInfo>, i32> {
    get_post_pagination(db, page, page_size, filter_user_id, current_user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get post pagination: {}", e);
            constants::SYSTEM_ERROR
        })
}

pub async fn like_post_service(
    db: &DatabaseConnection,
    token: &str,
    post_id: i32,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    // 检查帖子是否存在
    let is_liked = check_user_liked_post(db, claims.id, post_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check if user liked post: {}", e);
            constants::SYSTEM_ERROR
        })?;

    if is_liked {
        // 已点赞，取消点赞
        unlike_post(db, claims.id, post_id).await.map_err(|e| {
            tracing::error!("Failed to unlike post: {}", e);
            constants::SYSTEM_ERROR
        })
    } else {
        // 未点赞，添加点赞
        like_post(db, claims.id, post_id).await.map_err(|e| {
            tracing::error!("Failed to like post: {}", e);
            constants::SYSTEM_ERROR
        })
    }
}

pub async fn delete_post_service(
    db: &DatabaseConnection,
    token: &str,
    post_id: i32,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let deleted = delete_content(db, post_id, claims.id).await.map_err(|e| {
        tracing::error!("Failed to delete post: {}", e);
        constants::SYSTEM_ERROR
    })?;

    if deleted {
        Ok(())
    } else {
        Err(constants::FORBIDDEN)
    }
}

pub async fn update_post_service(
    db: &DatabaseConnection,
    token: &str,
    post_id: i32,
    new_content: String,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let updated = update_content(db, post_id, claims.id, new_content)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update post: {}", e);
            constants::SYSTEM_ERROR
        })?;

    if updated {
        Ok(())
    } else {
        Err(constants::FORBIDDEN)
    }
}

pub async fn create_comment_service(
    db: &DatabaseConnection,
    token: &str,
    content: String,
    post_id: i32,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let new_comment = NewComment {
        content,
        post_id,
        user_id: claims.id,
    };

    add_comment(db, new_comment).await.map_err(|e| {
        tracing::error!("Failed to add comment to database: {}", e);
        constants::SYSTEM_ERROR
    })?;

    Ok(())
}

pub async fn get_comments_service(
    db: &DatabaseConnection,
    post_id: i32,
    page: u64,
    page_size: u64,
) -> Result<PaginationData<CommentInfo>, i32> {
    get_comments_by_post_id(db, post_id, page, page_size)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get comments: {}", e);
            constants::SYSTEM_ERROR
        })
}

pub async fn delete_comment_service(
    db: &DatabaseConnection,
    token: &str,
    comment_id: i32,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let deleted = db_delete_comment(db, comment_id, claims.id).await.map_err(|e| {
        tracing::error!("Failed to delete comment: {}", e);
        constants::SYSTEM_ERROR
    })?;

    if deleted {
        Ok(())
    } else {
        Err(constants::FORBIDDEN)
    }
}

pub async fn update_comment_service(
    db: &DatabaseConnection,
    token: &str,
    comment_id: i32,
    new_content: String,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let updated = db_update_comment(db, comment_id, claims.id, new_content)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update comment: {}", e);
            constants::SYSTEM_ERROR
        })?;

    if updated {
        Ok(())
    } else {
        Err(constants::FORBIDDEN)
    }
}
