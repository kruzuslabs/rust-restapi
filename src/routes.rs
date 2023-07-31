use actix_web::{get, HttpResponse, Responder};

/// Main root handler
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!(
      {
            "msg":
            "hello world"
        }
    ))
}
