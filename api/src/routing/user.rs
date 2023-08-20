use axum::{
    extract::{Path, State},
    Json,
};
use nnc_entity::user;
use nnc_service::{Mutation, Query};
use sea_orm::DbConn;
use serde::Deserialize;

use super::AppError;

pub async fn list_users(db: State<DbConn>) -> Result<Json<Vec<user::Model>>, AppError> {
    Ok(Json(Query::list_users(&db).await?))
}

#[derive(Deserialize)]
pub struct CreateUserReqBody {
    name: String,
}

pub async fn create_user(
    db: State<DbConn>,
    Json(CreateUserReqBody { name }): Json<CreateUserReqBody>,
) -> Result<Json<user::Model>, AppError> {
    Ok(Json(Mutation::create_user(&db, name).await?))
}

pub async fn get_user_by_id(
    Path(id): Path<String>,
    db: State<DbConn>,
) -> Result<Json<user::Model>, AppError> {
    Ok(Json(
        Query::get_user_by_id(&db, id)
            .await?
            .ok_or(AppError::NotFound)?,
    ))
}
