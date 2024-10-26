mod controller;
mod sql_operations;
mod subscription;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .route(
                "/subscription",
                web::post().to(controller::create_subscription),
            )
            .route(
                "/unsubscribe",
                web::delete().to(controller::unsubscribe_from_channel),
            )
            .wrap(cors)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
