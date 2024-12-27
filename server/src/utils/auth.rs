use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const COST: u32 = DEFAULT_COST; // Adjustable hashing cost

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, COST)
}

pub fn verify_password(password: &str, hash_password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash_password)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, //User ID
    pub exp: usize,
}

pub fn generate_jwt(user_id: Uuid, secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

// //Find a user by email
// pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
//     sqlx::query_as!("SELECT * FROM \"user\" WHERE email = $1", email)
//         .fetch_optional(pool)
//         .await
// }

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn find_by_email(
        email: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"SELECT id, email, password FROM "user" WHERE email = $1"#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
