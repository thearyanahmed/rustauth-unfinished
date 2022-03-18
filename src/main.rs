mod config;
mod handlers;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;

use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("server configuration");

    info!("starting server at {}:{}",config.host,config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
