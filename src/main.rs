use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;

mod handlers;
use crate::handlers::register;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(register)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
