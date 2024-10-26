mod controller;
mod post;
mod sql_operations;

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
                "/posts/channel/{id}",
                web::get().to(controller::get_posts_by_channel),
            )
            .wrap(cors)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
