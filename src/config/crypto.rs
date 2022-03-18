use std::sync::Arc;
use color_eyre::Result;
use argonautica::Hasher;
use futures::compat::Future01CompatExt;
use eyre::eyre;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>
}

impl CryptoService {
    #[instrument(skip(self,password))]
    pub async fn hash_password(&self, password: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("hashing error: {:?}", err))
    }
}