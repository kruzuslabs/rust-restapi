extern crate validator_derive;

mod config;
mod handlers;
mod jwt_auth;
mod logger;
mod models;
mod response;
mod routes;

use std::io;
use std::thread::sleep;

use actix_cors::Cors;
// use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use logger::LoggerType;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::init();

    let mut retries = 0;
    let max_retries = 5;
    let retry_delay = Duration::from_secs(60); //each 1 min

    let pool = loop {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => {
                println!("✅ Connection to the database is successful! ✅");
                break pool;
            }

            Err(_) if retries < max_retries => {
                retries += 1;
                println!(
                    "❌ Database Connection tries: {} failed. Retrying in {} seconds...",
                    retries,
                    retry_delay.as_secs()
                );
                sleep(retry_delay);
            }

            Err(_) => {
                eprintln!("❌ Database connection attempts exhausted. Exiting...");
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    "Database connection error",
                ));
            }
        }
    };

    handlers::user::register::user();

    logger::log(LoggerType::Trace);

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
    })
    .bind("[::1]:8000")?
    .run()
    .await
}
