mod channel;
mod controller;
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
                "/channels/owner/{id}",
                web::get().to(controller::get_channels_created_by_user),
            )
            .route("/channels/all", web::get().to(controller::get_all_channels))
            .route(
                "/subscriptions/user/{id}",
                web::get().to(controller::get_subscriptions_by_user),
            )
            .route(
                "/categories/all",
                web::get().to(controller::get_all_categories),
            )
            .route(
                "/channel/create",
                web::post().to(controller::create_channel),
            )
            .route(
                "/channel/update/{id}",
                web::put().to(controller::update_channel),
            )
            .route(
                "/channel/delete/{id}", 
                web::delete().to(controller::delete_channel),
            )
            .wrap(cors)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
