use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;


use crate::utils::auth::{hash_password, verify_password, generate_jwt};
use crate::models::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInput {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LogInput {
    email: String,
    password: String,
}


pub async fn register_user(
    input: web::Json<RegisterInput>,
    pool: web::Data<PgPool>,
    secret: web::Data<String>,
) -> impl Responder {
    let hash_password = match hash_password(&input.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    match User::creeate_user(&input.name, &input.email, &hash_password, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

pub async fn login_user(
    input: web::Json<LogInput>,
    pool:web::Data<PgPool>,
    secret: web::Data<String>,
) -> impl Responder {
    if let Ok(Some(user)) = User::find_by_email(&input.email, &pool).await {
        if verify_password(&input.password, &user.password).unwrap_or(false) {
            match generate_jwt(user.id, secret.get_ref()) {
                Ok(token) => return HttpResponse::Ok().json(token),
                Err(_) => return HttpResponse::InternalServerError().body("Failed to generate token"),
            }
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}