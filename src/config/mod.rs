use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("load conf from env")
    }
}