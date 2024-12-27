use crate::services::login::{self, LogInput};
use crate::services::register::{self, RegisterPayload};
use actix_web::{post, web, Responder};
use sqlx::{Pool, Postgres};

#[post("/signup")]
pub async fn signup(
    payload: web::Json<RegisterPayload>,
    pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    register::register_user(payload, pool).await
}

//const  secret_in = web::Data::new("ibrahimmusharrafaudu").to_string();

#[post("/login")]
pub async fn login_in(
    payload: web::Json<LogInput>,
    pool: web::Data<Pool<Postgres>>,
    secret_in: web::Data<String>,
) -> impl Responder {
    login::login_user(payload, pool, secret_in).await
}
