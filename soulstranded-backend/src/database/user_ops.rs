use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QuerySelect, QueryTrait, Set, sea_query::Expr,
};
use serde::{Deserialize, Serialize};

use crate::entity::users;
use crate::handlers::user::UpdateUserInfoRequest;

#[derive(Deserialize, Serialize, FromQueryResult)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
}

impl From<users::Model> for UserInfo {
    fn from(model: users::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            username: model.username,
            avatar: model.avatar,
        }
    }
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn add_user(
    db: &DatabaseConnection,
    user: NewUser,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = users::ActiveModel {
        username: Set(user.username.to_string()),
        email: Set(user.email.to_string()),
        password: Set(user.password.to_string()),
        ..Default::default()
    };
    user.insert(db).await?;

    Ok(())
}

pub async fn check_email_registered(
    db: &DatabaseConnection,
    email: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let count = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .count(db)
        .await?;

    Ok(count > 0)
}

pub async fn get_password_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let password = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .select_only()
        .column(users::Column::Password)
        .into_tuple::<String>()
        .one(db)
        .await?;

    Ok(password)
}

pub async fn update_password_by_email(
    db: &DatabaseConnection,
    email: &str,
    encrypted_password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    users::Entity::update_many()
        .filter(users::Column::Email.eq(email))
        .col_expr(users::Column::Password, Expr::value(encrypted_password))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get_user_info_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<UserInfo>, Box<dyn std::error::Error>> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Username)
        .column(users::Column::Email)
        .column(users::Column::Avatar)
        .into_model::<UserInfo>()
        .one(db)
        .await?;

    Ok(user)
}

pub async fn update_user_info_by_email(
    db: &DatabaseConnection,
    email: &str,
    user_info: UpdateUserInfoRequest,
) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or("User not found")?;

    let mut active_user: users::ActiveModel = user.into();

    if let Some(username) = user_info.username {
        active_user.username = Set(username);
    }
    if let Some(avatar) = user_info.avatar {
        active_user.avatar = Set(Some(avatar));
    }

    let updated_user: users::Model = active_user.update(db).await?;

    Ok(UserInfo::from(updated_user))
}

pub async fn get_user_post_count(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<u64, Box<dyn std::error::Error>> {
    use crate::entity::contents;

    let count = contents::Entity::find()
        .filter(contents::Column::UserId.eq(user_id))
        .count(db)
        .await?;

    Ok(count)
}

pub async fn get_user_received_likes_count(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<u64, Box<dyn std::error::Error>> {
    use crate::entity::contents;
    use crate::entity::likes;

    let count = likes::Entity::find()
        .filter(
            likes::Column::PostId.in_subquery(
                contents::Entity::find()
                    .select_only()
                    .column(contents::Column::Id)
                    .filter(contents::Column::UserId.eq(user_id))
                    .into_query(),
            ),
        )
        .count(db)
        .await?;

    Ok(count)
}
