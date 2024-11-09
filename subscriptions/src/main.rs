mod routes;
mod sql_operations;
mod subscription;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        #[derive(OpenApi)]
        #[openapi(
            paths(routes::create_subscription, routes::unsubscribe_from_channel,),
            components(schemas(subscription::Subscription))
        )]
        struct ApiDoc;

        let openapi = ApiDoc::openapi();

        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(routes::create_subscription)
            .service(routes::unsubscribe_from_channel)
            .wrap(cors)
    })
    .bind("0.0.0.0:8082")?
    .run()
    .await
}
