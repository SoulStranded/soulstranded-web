pub mod comment_ops;
pub mod like_ops;
pub mod post_ops;
pub mod user_ops;

pub use comment_ops::*;
pub use like_ops::*;
pub use post_ops::*;
pub use user_ops::*;

use sea_orm::{Database, DatabaseConnection};

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let conn = Database::connect(database_url).await?;

    Ok(conn)
}
