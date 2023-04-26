use actix_web::dev::Server;
use actix_web::{
    cookie::time::Duration, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
