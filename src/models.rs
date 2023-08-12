use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//need data validation,

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,

    pub username: String,

    pub hashed_password: String,

    pub total_posts: Option<i32>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
struct Resource {
    id: i32,
    title: String,
    description: Option<String>,
    content: String,
    author_id: Uuid,
    created_at: NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}
