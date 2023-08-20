use nanoid::nanoid;
use nnc_entity::{user, wallet};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    fn generate_id() -> String {
        nanoid!(10) // this still provides 64^10 possible IDs
    }

    pub async fn create_user(db: &DbConn, name: impl ToString) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            id: Set(Self::generate_id()),
            name: Set(name.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn create_wallet(
        db: &DbConn,
        owner_id: impl ToString,
    ) -> Result<wallet::Model, DbErr> {
        wallet::ActiveModel {
            id: Set(Self::generate_id()),
            owner_id: Set(owner_id.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
