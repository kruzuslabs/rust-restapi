//TODO more

pub mod crud {

    use actix_web::{get, HttpResponse, Responder};
    use serde_json::json;

    #[get("/resources")]
    pub async fn create() -> impl Responder {
        HttpResponse::Ok().json(json!({"status": "success"}))
    }

    #[get("/resources/id")]
    pub async fn update_by_id() -> impl Responder {
        HttpResponse::Ok().json(json!({"status": "success"}))
    }

    #[get("/resource/id")]
    pub async fn delete_id() -> impl Responder {
        HttpResponse::Ok().json(json!({"status": "success"}))
    }

    #[get("/resources/user/id")]
    pub async fn read_all() -> impl Responder {
        HttpResponse::Ok().json(json!({"status": "success"}))
    }

    #[get("/posts")]
    pub async fn read_one() -> impl Responder {
        HttpResponse::Ok().json(json!({"status": "success"}))
    }
}
