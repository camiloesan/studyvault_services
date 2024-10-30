mod controller;
mod sql_operations;
mod auth;

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
            .route("/login", web::post().to(controller::login_user))
            .wrap(cors)
    })
    .bind("0.0.0.0:8085")?
    .run()
    .await
}
