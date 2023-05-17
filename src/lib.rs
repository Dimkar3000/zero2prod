use actix_web::{web, App, HttpServer};

pub mod configuration;
pub mod routes;

use crate::routes::*;

pub async fn run() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("failed to get configuration");
    dbg!(&config);

    let address = config.get_url_to_bind();

    println!("listening to http://{address}");

    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::get().to(subsribe))
    })
    .bind(address)?
    .run()
    .await
}
