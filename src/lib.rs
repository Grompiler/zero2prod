use actix_web::dev::Server;
use actix_web::web::Form;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subsribe", web::post().to(subsribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn subsribe(_form: Form<Subsriber>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct Subsriber {
    name: String,
    email: String,
}
