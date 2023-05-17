use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::types::{chrono::Utc, Uuid};

use crate::ApplicationState;

#[derive(Deserialize)]
pub struct SubscriptionFormDto {
    email: String,
    name: String,
}

pub async fn subsribe(
    form: web::Form<SubscriptionFormDto>,
    state: web::Data<ApplicationState>,
) -> HttpResponse {
    let state = state.get_ref();
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.database_connection)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
