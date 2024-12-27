use actix_web::{web, HttpResponse, Responder};
//use bcrypt::{hash, DEFAULT_COST};
use crate::utils::auth::{generate_jwt, hash_password, verify_password, User};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

/// Represents input data for login.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogInput {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    input: web::Json<LogInput>,
    pool: web::Data<Pool<Postgres>>,
    secret: web::Data<String>,
) -> impl Responder {
    if let Ok(Some(user)) = User::find_by_email(&input.email, &pool).await {
        if verify_password(&input.password, &user.password).unwrap_or(false) {
            match generate_jwt(user.id, secret.get_ref()) {
                Ok(token) => return HttpResponse::Ok().json(token),
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Failed to generate token")
                }
            }
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
