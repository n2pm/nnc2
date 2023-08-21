use axum::{extract::State, Json};
use nnc_entity::wallet;
use nnc_service::Query;
use sea_orm::DbConn;

use super::AppError;

pub async fn list_wallets(db: State<DbConn>) -> Result<Json<Vec<wallet::Model>>, AppError> {
    Ok(Json(Query::list_wallets(&db).await?))
}
