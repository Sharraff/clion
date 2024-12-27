use actix_web::{web, App, HttpServer};
//use sqlx::PgPool;
//use env;
use sqlx::postgres::PgPoolOptions;
use std::io;

//use crate::api::v1::auth;

mod api; // Declare the `api` module at the crate root
mod services;
mod utils; // Add this line to register the `utils` module // Register the `models` module

#[actix_web::main]
async fn main() -> io::Result<()> {
    //create a postgreSQL connection pool io::Result<()>
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ubuntu:Musharraff1@localhost:5432/clion")
        .await;

    let pool = match pool {
        Ok(pool) => {
            println!("Database pool created and cloned successfully.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to create database pool: {:?}", e);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Database connection failed",
            ));
        }
    };

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    //let pool = PgPool::connect(&database_url).await.unwrap();
    let secret = web::Data::new("ibrahimmusharrafaudu".to_string());

    HttpServer::new(move || {
        // Clone the pool for Actix Web
        let cloned_pool = pool.clone();

        App::new()
            .app_data(web::Data::new(cloned_pool))
            .app_data(secret.clone())
            .service(api::v1::auth::signup)
            .service(api::v1::auth::login_in)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// .route(
// "/register",
// web::post().to(services::register::register_user),
// .app_data(web::Data::new(jwt_secret.clone()))
// .route("/register", web::post().to(auth::register_user))
//.route("/login_user", web::post().to(auth::login_user))
