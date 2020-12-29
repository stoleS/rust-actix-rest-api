pub mod crypto;

use color_eyre::Result;
use crypto::CryptoService;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

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

        info!("Loading configuration...");

        let mut conf = config::Config::new();

        conf.merge(config::Environment::default())?;

        conf.try_into()
            .context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool...");

        PgPool::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating connection database pool")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
        }
    }
}
