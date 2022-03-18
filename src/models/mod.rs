use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id : Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub name : Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min=3, max=12))]
    pub username: String,
    #[validate(email,length(min=3, max=12))]
    pub email: String,
    #[validate(length(min=3, max=12))]
    pub password: String,
}
