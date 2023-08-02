mod logger;
mod models;
mod routes;
mod response;

use crate::logger::LoggerType;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //change this feels weird to type.
    logger::log(LoggerType::Trace);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::hello_user)
            .service(routes::root)
    })
    .bind("[::1]:8520")?
    .run()
    .await
}
