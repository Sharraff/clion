use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{Pool, Postgres};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    name: String,
    email: String,
    password: String,
}

pub async fn register_user(
    payload: web::Json<RegisterPayload>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    // Current timestamp as PrimitiveDateTime
    let now = OffsetDateTime::now_utc();

    let new_user = User {
        id: Uuid::new_v4(),
        name: payload.name.clone(),
        email: payload.email.clone(),
        password: hashed_password,
        created_at: Some(PrimitiveDateTime::new(now.date(), now.time())),
        updated_at: Some(PrimitiveDateTime::new(now.date(), now.time())),
    };

    // Saving the user to the database
    match sqlx::query!(
        r#"
        INSERT INTO "user" (id, name, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        new_user.id,
        new_user.name,
        new_user.email,
        new_user.password,
        new_user.created_at,
        new_user.updated_at,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(new_user),
        Err(e) => {
            eprintln!("Failed to insert user into database: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
}
