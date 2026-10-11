use chrono::{Local, SubsecRound};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

use crate::entity::likes;

pub async fn like_post(
    db: &DatabaseConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let like = likes::ActiveModel {
        user_id: Set(user_id),
        post_id: Set(post_id),
        created_at: Set(Local::now().naive_local().trunc_subsecs(0)),
        ..Default::default()
    };

    like.insert(db).await?;
    Ok(())
}

pub async fn unlike_post(
    db: &DatabaseConnection,
    user_id: i32,
    post_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    likes::Entity::delete_many()
        .filter(likes::Column::UserId.eq(user_id))
        .filter(likes::Column::PostId.eq(post_id))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn check_user_liked_post(
    db: &DatabaseConnection,
    user_id: i32,
    post_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let count = likes::Entity::find()
        .filter(likes::Column::UserId.eq(user_id))
        .filter(likes::Column::PostId.eq(post_id))
        .count(db)
        .await?;

    Ok(count > 0)
}
