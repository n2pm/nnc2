use axum::{
    extract::{Path, State},
    Json,
};
use nnc_service::{compound::UserWithWallets, Mutation, Query};
use sea_orm::DbConn;
use serde::Deserialize;

use super::AppError;

pub async fn list_users(db: State<DbConn>) -> Result<Json<Vec<UserWithWallets>>, AppError> {
    Ok(Json(Query::list_users_with_wallets(&db.0).await?))
}

#[derive(Deserialize)]
pub struct CreateUserReqBody {
    name: String,
}

pub async fn create_user(
    db: State<DbConn>,
    Json(CreateUserReqBody { name }): Json<CreateUserReqBody>,
) -> Result<Json<UserWithWallets>, AppError> {
    Ok(Json(
        Mutation::create_user_with_default_wallet(&db.0, name).await?,
    ))
}

pub async fn get_user_by_id(
    Path(id): Path<String>,
    db: State<DbConn>,
) -> Result<Json<UserWithWallets>, AppError> {
    Ok(Json(
        Query::get_user_by_id_with_wallets(&db.0, id)
            .await?
            .ok_or(AppError::NotFound)?,
    ))
}
