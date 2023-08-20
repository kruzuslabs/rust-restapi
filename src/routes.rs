use actix_web::web;

use crate::handlers;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(handlers::user::register)
        .service(handlers::user::login)
        .service(handlers::user::logout)
        .service(handlers::user::posts)
        .service(handlers::user::get_me);

    conf.service(scope);
}
