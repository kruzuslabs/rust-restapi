use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,

    #[serde(rename = "hashedPassword")]
    pub hashed_password: String,

    #[serde(rename = "totalPosts")]
    pub total_posts: i32,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Resources {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub content: String,

    #[serde(rename = "authorID")]
    pub author_id: Uuid,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
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