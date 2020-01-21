use super::requests::UserReq;
use super::Pool;
use actix_web::{post, web, Error, HttpResponse};

#[post("/user/register")]
pub async fn register(
    user: web::Json<UserReq>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || user.register(&pool.get().unwrap()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/user/login")]
pub async fn login(user: web::Json<UserReq>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || user.login(&pool.get().unwrap()))
        .await
        .map(|()| HttpResponse::Ok().body("Logged in!\n"))
        .map_err(|_| HttpResponse::InternalServerError().body("Login Failed\n"))?)
}
