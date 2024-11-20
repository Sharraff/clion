use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String, // Hashed password
}

impl User {
    pub async fn creeate_user(
        name: &str,
        email: &str,
        hash_password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO user (name, emai, password VALUES ($1 $2 $3) RETURN *)",
            name,
            email,
            hash_password
        )
        .fetch_one(pool)
        .await?
    }
    
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM user where email = $1",
            email
        )
        .fetch_optional(pool)
        .await?
    }
}