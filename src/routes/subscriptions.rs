use actix_web::web::Form;
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

pub async fn subscribe(_form: Form<Subsriber>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct Subsriber {
    name: String,
    email: String,
}
