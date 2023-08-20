mod config;
mod routing;
mod tracing;

use ::tracing::{debug, info};
use axum::Server;
use eyre::Context;
use nnc_migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::{config::Config, routing::make_router, tracing::set_up_tracing};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    set_up_tracing("nnc_api").context("Failed to set up tracing")?;

    debug!("Loading config");
    let Config {
        listen_addr,
        db_addr,
    } = Config::load().context("Failed to load config")?;

    debug!("Connecting to {db_addr}");
    let db = Database::connect(db_addr)
        .await
        .context("Failed to connect to the database")?;

    debug!("Applying migrations");
    Migrator::up(&db, None)
        .await
        .context("Failed to apply migrations")?;

    info!("Starting service on {listen_addr}");
    Server::bind(&listen_addr)
        .serve(make_router(db).into_make_service())
        .await
        .context("Failed to serve app")?;

    Ok(())
}
