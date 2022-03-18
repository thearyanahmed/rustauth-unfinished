use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("loading config from env.");

        let mut config = config::Config::new();

        config.merge(config::Environment::default())?;

        config.try_into()
            .context("load conf from env")
    }
}