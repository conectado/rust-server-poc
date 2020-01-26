use crate::models::requests::UserReq;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{error::ErrorUnauthorized, get, post, web, Error, HttpResponse};

#[post("/register")]
pub async fn register(
    user: web::Json<UserReq>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || user.register(&pool.get().unwrap()))
        .await
        .map(|_| HttpResponse::Ok().body("User registered!\n"))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/login")]
pub async fn login(
    identity: Identity,
    user: web::Json<UserReq>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || user.login(&pool.get().unwrap()))
        .await
        .map(|stored_user| {
            // The stored session only validates username
            identity.remember(stored_user.username.clone());
            HttpResponse::Ok().body("Logged in!\n")
        })
        .map_err(|_| {
            // A failed login invalidates cookie just for the sake of testinge
            identity.forget();
            ErrorUnauthorized("Invalid credentials\n")
        })?)
}

#[get("/whoami")]
pub async fn whoami(id: Identity) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        Ok(HttpResponse::Ok().body(id))
    } else {
        Err(ErrorUnauthorized("A request has no name\n"))
    }
}
