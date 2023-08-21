use axum::{
    extract::{Path, State},
    Json,
};
use nnc_entity::wallet;
use nnc_service::{compound::WalletWithOwner, Query};
use sea_orm::DbConn;

use super::AppError;

pub async fn list_wallets(db: State<DbConn>) -> Result<Json<Vec<WalletWithOwner>>, AppError> {
    Ok(Json(Query::list_wallets_with_owners(&db.0).await?))
}

pub async fn get_wallet_by_id(
    db: State<DbConn>,
    Path(id): Path<String>,
) -> Result<Json<WalletWithOwner>, AppError> {
    Ok(Json(
        Query::get_wallet_by_id_with_owner(&db.0, id)
            .await?
            .ok_or(AppError::NotFound)?,
    ))
}
