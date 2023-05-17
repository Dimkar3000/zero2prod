use actix_web::{web, App, HttpServer};

use sqlx::{Connection, PgConnection};
pub mod configuration;
pub mod routes;

use crate::routes::*;

pub struct ApplicationState {
    pub database_connection: PgConnection,
}

impl ApplicationState {
    pub fn new(database_connection: PgConnection) -> Self {
        ApplicationState {
            database_connection,
        }
    }
}

pub async fn run() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("failed to get configuration");

    let address = config.get_url_to_bind();
    let connection = PgConnection::connect(&config.database.get_connection_string())
        .await
        .expect("failed to connect to the database.");
    let app_state = web::Data::new(ApplicationState::new(connection));

    println!("listening to http://{address}");

    HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::get().to(subsribe))
            .app_data(app_state.clone())
    })
    .bind(address)?
    .run()
    .await
}
