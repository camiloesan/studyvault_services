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
            .route("/comment/all/{id}", web::get().to(controller::get_all_comments_by_post_id))
            .route("/comment", web::post().to(controller::comment_post))
            .route("/comment/update/{id}", web::put().to(controller::update_existing_comment))
            .route("/comment/delete/{id}", web::delete().to(controller::delete_existing_comment))
            .wrap(cors)
    })
    .bind("0.0.0.0:8084")?
    .run()
    .await
}
