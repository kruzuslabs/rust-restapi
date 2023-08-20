use crate::{
    jwt_auth,
    models::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    response::FilteredUser,
    AppState,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::Row;
use validator::Validate;

//test
#[get("/posts")]
async fn posts(_: jwt_auth::JwtMiddleware) -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success"}))
}

#[post("/auth/register")]
async fn register(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    //validate Data first

    if let Err(validation_error) = body.validate() {
        return HttpResponse::BadRequest().json(validation_error);
    }

    //check if user exists
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
        .bind(body.username.to_string())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    // if it does send an error
    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    //generatre salt string using Os Random
    let salt = SaltString::generate(&mut OsRng);

    //getting the password user written, and hashing it
    let hashed_password: String = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    // query the user
    let query_result: Result<User, _> = sqlx::query_as!(
        User,
        "INSERT INTO users (username,hashed_password) VALUES ($1, $2) RETURNING *",
        body.username.to_string(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await;

    // pattern matching for the user
    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_user_record(&user)
            })});

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","messageLMAÕ": format!("{:?}", e)}));
        }
    }
}

#[post("/auth/login")]
async fn login(body: web::Json<LoginUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        body.username.to_string()
    )
    .fetch_optional(&data.db)
    .await
    .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.hashed_password).unwrap();

        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid password"}));
    }

    let user = query_result.unwrap();

    let now = Utc::now();

    let iat = now.timestamp() as usize;

    let exp = (now + Duration::minutes(30)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(30 * 60, 0))
        .http_only(true)
        .secure(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
async fn logout(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();

    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    HttpResponse::Ok().json(json_response)
}
pub fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id,
        username: user.username.to_string(),
        createdAt: user.created_at,
        updatedAt: user.updated_at,
        total_posts: user.total_posts,
    }
}
