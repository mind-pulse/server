mod access_log;
mod config;
mod db;
mod error;
mod routes;
mod scale;
mod server;
mod statistics;
mod telemetry;

#[macro_use]
extern crate tracing;

use crate::config::ServerConfig;
use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ServerConfig::from_env_and_args();

    // NOTE: _guard must be a top-level variable
    let _guard = telemetry::init();

    let pool = db::connect(&config.db_path).await?;
    db::migrate(&pool).await?;

    server::serve(pool, config.bind_addr).await;

    Ok(())
}
