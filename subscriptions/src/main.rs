mod model;
mod repository;
mod routes;
mod sql_repo;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validate_jwt;
use sql_repo::MySQLSubscriptionsRepository;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url =
        std::env::var("DATABASE_URL").expect("Couldn't get secret key from cargo environment");
    let repo: MySQLSubscriptionsRepository = MySQLSubscriptionsRepository::new(&url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        #[derive(OpenApi)]
        #[openapi(
            paths(routes::create_subscription, routes::unsubscribe_from_channel,),
            components(schemas(model::Subscription))
        )]
        struct ApiDoc;

        let openapi = ApiDoc::openapi();

        App::new()
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .app_data(web::Data::new(repo.clone()))
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
