use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;
use std::io;

use crate::api::v1::auth;

mod api; // Declare the `api` module at the crate root
mod utils; // Add this line to register the `utils` module
mod models; // Register the `models` module


#[actix_web::main]
async fn main() -> io::Result<()> {
    // create a postgreSQL connection pool
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://ubuntu:Musharraff1@localhost:5432/clion")
    //     .await?;

    // println!("connecting to the database!!");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .route("/register", web::post().to(auth::register_user))
            .route("/login_user", web::post().to(auth::login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
