use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters))]
pub async fn confirm(parameters: web::Query<Parameters>) -> HttpResponse {
    let _ = &parameters.subscription_token;
    HttpResponse::Ok().finish()
}
