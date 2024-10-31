mod sql_operations;
mod comment;
mod controller;

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
            .route("/comment/all", web::get().to(controller::get_all_comments_by_post_id))
            .route("/comment", web::post().to(controller::comment_post))
            .wrap(cors)
    })
    .bind("0.0.0.0:8084")?
    .run()
    .await
}
