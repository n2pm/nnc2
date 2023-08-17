mod config;
mod database;
mod model;
mod routing;
mod tracing;

use ::tracing::info;
use axum::Server;
use eyre::Context;

use crate::{
    config::Config, database::get_db_client, routing::make_router, tracing::set_up_tracing,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    set_up_tracing("notnetcoin").context("Failed to set up tracing")?;

    let Config { listen_addr } = Config::load().context("Failed to load config")?;

    let db = get_db_client()
        .await
        .context("Failed to connect to the database")?;

    let app = make_router(db);

    info!("Starting to listen on {listen_addr}");

    Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to serve app")?;

    Ok(())
}
