use chrono::{Local, SubsecRound};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::entity::comments;
use crate::entity::users;
use crate::utils::serializer::serialize_naive_datetime;

#[derive(Deserialize, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    pub id: i32,
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub updated_at: chrono::NaiveDateTime,
    pub username: String,
    pub avatar: Option<String>,
}

pub struct NewComment {
    pub content: String,
    pub post_id: i32,
    pub user_id: i32,
}

pub async fn add_comment(
    db: &DatabaseConnection,
    comment: NewComment,
) -> Result<(), Box<dyn std::error::Error>> {
    let comment = comments::ActiveModel {
        content: Set(comment.content.to_string()),
        post_id: Set(comment.post_id),
        user_id: Set(comment.user_id),
        created_at: Set(Local::now().naive_local().trunc_subsecs(0)),
        updated_at: Set(Local::now().naive_local().trunc_subsecs(0)),
        ..Default::default()
    };
    comment.insert(db).await?;

    Ok(())
}

fn build_comment_info(comment: comments::Model, user: users::Model) -> CommentInfo {
    CommentInfo {
        id: comment.id,
        content: comment.content,
        post_id: comment.post_id,
        user_id: comment.user_id,
        created_at: comment.created_at,
        updated_at: comment.updated_at,
        username: user.username,
        avatar: user.avatar,
    }
}

pub async fn get_comments_by_post_id(
    db: &DatabaseConnection,
    post_id: i32,
    page: u64,
    page_size: u64,
) -> Result<super::PaginationData<CommentInfo>, Box<dyn std::error::Error>> {
    let comments = comments::Entity::find()
        .filter(comments::Column::PostId.eq(post_id))
        .find_also_related(users::Entity)
        .order_by_desc(comments::Column::CreatedAt)
        .paginate(db, page_size);

    let total = comments.num_items().await?;
    let comment_models = comments.fetch_page(page - 1).await?;

    let mut comment_infos = Vec::new();
    for (comment, user) in comment_models {
        if let Some(user) = user {
            let comment_info = build_comment_info(comment, user);
            comment_infos.push(comment_info);
        }
    }

    Ok(super::PaginationData {
        page,
        page_size,
        total,
        data: comment_infos,
    })
}

pub async fn delete_comment(
    db: &DatabaseConnection,
    comment_id: i32,
    user_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let comment = comments::Entity::find_by_id(comment_id).one(db).await?;

    match comment {
        Some(comment) if comment.user_id == user_id => {
            comment.delete(db).await?;
            Ok(true)
        }
        Some(_) => Ok(false),
        None => Ok(false),
    }
}

pub async fn update_comment(
    db: &DatabaseConnection,
    comment_id: i32,
    user_id: i32,
    new_content: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let comment = comments::Entity::find_by_id(comment_id).one(db).await?;

    match comment {
        Some(comment) if comment.user_id == user_id => {
            let mut comment: comments::ActiveModel = comment.into();
            comment.content = Set(new_content);
            comment.updated_at = Set(Local::now().naive_local().trunc_subsecs(0));
            comment.update(db).await?;
            Ok(true)
        }
        Some(_) => Ok(false),
        None => Ok(false),
    }
}
