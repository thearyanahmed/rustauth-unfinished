pub mod crypto;

use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;
use tracing::{info, instrument};
use sqlx::PgPool;
use std::time::Duration;
use crate::config::crypto::CryptoService;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
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

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("creating database connection pool");

        PgPool::builder()
            .connect_timeout(Duration::from_secs(30))
            .build(&*self.database_url)
            .await
            .context("creating db connection pool")
    }

    pub async fn hashing(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone())
        }
    }
}

