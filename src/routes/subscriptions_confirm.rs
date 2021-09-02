//! src/routes/subscriptions_confirm.rs

use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
    _subscriptions_token: String
}

#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(_parameters)
)]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
