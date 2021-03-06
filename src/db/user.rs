use sqlx::{PgPool, postgres::PgQueryAs};
use std::sync::Arc;
use crate::models::{NewUser, User};
use crate::config::crypto::CryptoService;
use color_eyre::Result;

pub struct UserRepository {
    pool: Arc<PgPool>
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    pub async fn create(&self, new_user: NewUser, crypto_svc: &CryptoService) -> Result<User> {
        let password_hash = crypto_svc.hash_password(new_user.password).await?;

        let user = sqlx::query_as::<_, User>(
            "insert into users (username, email, password) value ($1, $2, $3) returning *"
        )
            .bind(new_user.username)
            .bind(new_user.email)
            .bind(password_hash)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user)
    }
}