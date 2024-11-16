mod controller;
mod sql_operations;
mod auth;
mod email_operations;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
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
                paths(
                    controller::login_user,
                    controller::request_verification,
                    controller::verify_code,
                ),
                components(schemas(auth::LoginData, auth::VerificationRequest))
            )]
            struct ApiDoc;
    
            let openapi = ApiDoc::openapi();

        App::new()
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(controller::login_user)
            .service(controller::request_verification)
            .service(controller::verify_code)
    })
    .bind("0.0.0.0:8085")?
    .run()
    .await
}
