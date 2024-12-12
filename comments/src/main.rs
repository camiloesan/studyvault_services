mod comment;
mod controller;
mod sql_operations;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
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
                controller::comment_post,
                controller::get_all_comments_by_post_id,
                controller::update_existing_comment,
                controller::delete_existing_comment,
            ),
            components(schemas(
                comment::CommentToInsert,
                comment::Comment,
                comment::CommentToUpdate
            ))
        )]
        struct ApiDoc;

        let openapi = ApiDoc::openapi();

        App::new()
            .wrap(HttpAuthentication::bearer(validate_jwt))
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(controller::comment_post)
            .service(controller::get_all_comments_by_post_id)
            .service(controller::update_existing_comment)
            .service(controller::delete_existing_comment)
            .service(controller::get_avg_rating)
    })
    .bind("0.0.0.0:8084")?
    .run()
    .await
}
