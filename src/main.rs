mod logger;
mod routes;

use crate::logger::LoggerType;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //change this feels weird to type.
    logger::log(LoggerType::Warning);


    println!("🚀 Server started successfully");

    HttpServer::new(move || App::new().wrap(Logger::default()).service(routes::root))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
