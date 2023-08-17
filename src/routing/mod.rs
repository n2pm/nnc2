mod account;
mod user;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use thiserror::Error;
use tracing::error;

use self::user::user;
use crate::database::Db;

pub fn make_router(db: Db) -> Router {
    Router::new()
        .route("/users/:name", get(user))
        // .route("/users/:name/accounts", get(user_accounts))
        .with_state(db)
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Eyre(#[from] eyre::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{self}");
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found.".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong. Please consult the logs."),
            ),
        }
        .into_response()
    }
}
