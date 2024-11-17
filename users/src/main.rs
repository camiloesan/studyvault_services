mod controller;
mod sql_operations;
mod user;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validate_jwt;
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
                controller::get_all_emails,
                controller::register_new_user,
                controller::update_existing_user,
                controller::delete_existing_user,
                controller::get_user_name_by_id,
                controller::update_user_password,
            ),
            components(schemas(user::UserName, user::RegisterRequest, user::UserToUpdate, user::PasswordToUpdate))
        )]
        struct ApiDoc;

        let openapi = ApiDoc::openapi();

        App::new()
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(controller::get_all_emails)
            .service(controller::register_new_user)
            .service(controller::update_user_password)
            .service(
                actix_web::web::scope("")
                    .wrap(HttpAuthentication::bearer(validate_jwt))
                    .service(controller::update_existing_user)
                    .service(controller::delete_existing_user)
                    .service(controller::get_user_name_by_id),
            )
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}
