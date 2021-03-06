#[macro_use]
extern crate diesel;
use crate::handlers::{login, register, whoami};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use listenfd::ListenFd;
use rand::{thread_rng, Rng};

mod handlers;
mod models;
mod password_manager;
mod schema;

pub type DBType = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DBType>>;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

fn get_connection_pool() -> Pool {
    let db_url =
        std::env::var("DATABASE_URL").expect("Please setup DATABASE_URL in .env in root folder");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build db connection pool")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let mut listenfd = ListenFd::from_env();

    let db_pool = get_connection_pool();
    let mut arr = [0u8; 32];
    thread_rng().fill(&mut arr);
    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&arr)
                    .name("server-auth")
                    .secure(false),
            ))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(
                web::scope("/user")
                    .service(register)
                    .service(login)
                    .service(whoami),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
