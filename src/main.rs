#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use tracing::info;
use color_eyre::Result;
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;

use crate::config::Config;
use crate::handlers::app_config;
use crate::config::crypto::CryptoService;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("server configuration");

    let pool = config.db_pool().await.expect("db config");

    let crypto_service : CryptoService = config.hashing().await;

    info!("starting server at {}:{}",config.host,config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
            .data(pool.clone())
            .data(crypto_service.clone())
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
