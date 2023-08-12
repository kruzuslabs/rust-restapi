extern crate validator_derive;

mod config;
mod jwt_auth;
mod logger;
mod models;
mod response;
mod routes;


use std::io;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use logger::LoggerType;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::log(LoggerType::Trace);

    dotenv().ok();

    let config = Config::init();

    //work on this later
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        
        .await
        
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful! ✅");
            pool
        }
        Err(_) => {

            return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "Database connection error"));
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("[::1]:8000")?
    .run()
    .await
}
