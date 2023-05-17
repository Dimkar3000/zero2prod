use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionFormDto {
    pub email: String,
    pub name: String,
}

pub async fn subsribe(_form: web::Form<SubscriptionFormDto>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
