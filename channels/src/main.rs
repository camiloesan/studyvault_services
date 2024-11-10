mod channel;
mod controller;
mod sql_operations;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use auth::validate_jwt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        #[derive(OpenApi)]
        #[openapi(
            paths(
                controller::get_all_channels,
                controller::get_channels_created_by_user,
                controller::get_subscriptions_by_user,
            ),
            components(schemas(channel::Channel, channel::ChannelUpdateData,))
        )]
        struct ApiDoc;

        let openapi = ApiDoc::openapi();

        App::new()
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(controller::get_all_channels)
            .service(controller::get_channels_created_by_user)
            .service(controller::get_subscriptions_by_user)
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
            .route(
                "/channel/name/{id}",
                web::get().to(controller::get_channel_name_by_id),
            )
            .route(
                "/creator/channel/{id}",
                web::get().to(controller::get_creator_id_by_channel_id),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
