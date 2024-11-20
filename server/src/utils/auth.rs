use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Deserialize, Serialize};
// use std::time::Duration;
use chrono::Utc;
use chrono::Duration;

const COST: u32 = DEFAULT_COST; // Adjustable hashing cost


pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, COST)
}

pub fn verify_password(password: &str, hash_password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash_password)    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, //User ID
    pub exp: usize,
}

pub fn generate_jwt(user_id: i32, secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
    .map(|data| data.claims)
}