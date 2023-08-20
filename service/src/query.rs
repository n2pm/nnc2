use nnc_entity::user::{self, Entity as User};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn list_users(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(db).await
    }

    pub async fn get_user_by_id(
        db: &DbConn,
        id: impl ToString,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id.to_string()).one(db).await
    }
}
