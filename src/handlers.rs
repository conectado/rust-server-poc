use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn register() -> impl Responder {
    HttpResponse::Ok()
}
