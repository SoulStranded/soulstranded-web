use chrono::{Local, SubsecRound};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::entity::comments;
use crate::entity::contents;
use crate::entity::likes;
use crate::entity::users;
use crate::utils::serializer::serialize_naive_datetime;

#[derive(Deserialize, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct PostInfo {
    pub id: i32,
    pub content: String,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub pub_time: chrono::NaiveDateTime,
    #[serde(serialize_with = "serialize_naive_datetime")]
    pub update_time: chrono::NaiveDateTime,
    pub username: String,
    pub avatar: Option<String>,
    pub like_count: i32,
    pub is_liked: bool,
    pub comment_count: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationData<T> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub data: Vec<T>,
}

pub struct NewContent {
    pub content: String,
    pub user_id: i32,
}

pub async fn add_content(
    db: &DatabaseConnection,
    content: NewContent,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = contents::ActiveModel {
        content: Set(content.content.to_string()),
        pub_time: Set(Local::now().naive_local().trunc_subsecs(0)),
        update_time: Set(Local::now().naive_local().trunc_subsecs(0)),
        user_id: Set(content.user_id),
        ..Default::default()
    };
    content.insert(db).await?;

    Ok(())
}

async fn calculate_post_like_info(
    db: &DatabaseConnection,
    post_id: i32,
    user_id: Option<i32>,
) -> Result<(i32, bool), Box<dyn std::error::Error>> {
    let like_count = likes::Entity::find()
        .filter(likes::Column::PostId.eq(post_id))
        .count(db)
        .await? as i32;

    let is_liked = if let Some(user_id) = user_id {
        likes::Entity::find()
            .filter(likes::Column::PostId.eq(post_id))
            .filter(likes::Column::UserId.eq(user_id))
            .count(db)
            .await?
            > 0
    } else {
        false
    };

    Ok((like_count, is_liked))
}

async fn calculate_post_comment_count(
    db: &DatabaseConnection,
    post_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let comment_count = comments::Entity::find()
        .filter(comments::Column::PostId.eq(post_id))
        .count(db)
        .await? as i32;
    Ok(comment_count)
}

fn build_post_info(
    content: contents::Model,
    user: users::Model,
    like_count: i32,
    is_liked: bool,
    comment_count: i32,
) -> PostInfo {
    PostInfo {
        id: content.id,
        content: content.content,
        pub_time: content.pub_time,
        update_time: content.update_time,
        username: user.username,
        avatar: user.avatar,
        like_count,
        is_liked,
        comment_count,
    }
}

pub async fn get_post_pagination(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
    filter_user_id: Option<i32>,
    current_user_id: Option<i32>,
) -> Result<PaginationData<PostInfo>, Box<dyn std::error::Error>> {
    let mut select = contents::Entity::find()
        .find_also_related(users::Entity)
        .order_by_desc(contents::Column::PubTime);

    if let Some(uid) = filter_user_id {
        select = select.filter(contents::Column::UserId.eq(uid));
    }

    let posts = select.paginate(db, page_size);

    let total = posts.num_items().await?;
    let post_models = posts.fetch_page(page - 1).await?;

    let mut post_infos = Vec::new();
    for (content, user) in post_models {
        if let Some(user) = user {
            let (like_count, is_liked) =
                calculate_post_like_info(db, content.id, current_user_id).await?;
            let comment_count = calculate_post_comment_count(db, content.id).await?;
            let post_info = build_post_info(content, user, like_count, is_liked, comment_count);
            post_infos.push(post_info);
        }
    }

    Ok(PaginationData {
        page,
        page_size,
        total,
        data: post_infos,
    })
}

pub async fn delete_content(
    db: &DatabaseConnection,
    post_id: i32,
    user_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let post = contents::Entity::find_by_id(post_id).one(db).await?;

    match post {
        Some(post) if post.user_id == user_id => {
            post.delete(db).await?;
            Ok(true)
        }
        Some(_) => Ok(false),
        None => Ok(false),
    }
}

pub async fn update_content(
    db: &DatabaseConnection,
    post_id: i32,
    user_id: i32,
    new_content: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let post = contents::Entity::find_by_id(post_id).one(db).await?;

    match post {
        Some(post) if post.user_id == user_id => {
            let mut active_post: contents::ActiveModel = post.into();
            active_post.content = Set(new_content);
            active_post.update_time = Set(Local::now().naive_local().trunc_subsecs(0));
            active_post.update(db).await?;
            Ok(true)
        }
        Some(_) => Ok(false),
        None => Ok(false),
    }
}
