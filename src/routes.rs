use actix_web::{get, web, HttpResponse, Responder};

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

#[get("hello/{user}")]
async fn hello_user(user: web::Path<String>) -> impl Responder {
    if let Ok(parse_id) = user.parse::<i32>() {
        HttpResponse::Ok().json(serde_json::json!(
            {
                "msg": format!("hello user ID {}", parse_id)
            }
        ))
    } else {
        HttpResponse::Ok().json(serde_json::json!(
            {
                "error": format!("ID: {}", user)
            }
        ))
    }
}
