mod config;
mod handlers;

use tracing::info;
use color_eyre::Result;
use actix_web::{HttpServer, App};
use actix_web::middleware::Logger;

use crate::config::Config;
use crate::handlers::app_config;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("server configuration");

    info!("starting server at {}:{}",config.host,config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
