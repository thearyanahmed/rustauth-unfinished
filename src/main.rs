mod config;

use color_eyre::Result;
use crate::config::Config;
use actix_web::HttpServer;

#[actix_rt::main]
fn main() -> Result<()>{
    let config = Config::from_env()
        .expect("server configuration");
    

    Ok(())
}
