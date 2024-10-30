mod controller;
mod sql_operations;
mod user;

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
            .route("/user/email/all", web::get().to(controller::get_all_emails))
            .route("/register", web::post().to(controller::register_new_user))
            .route("/update/{id}", web::put().to(controller::update_existing_user))
            .route("/delete/{id}", web::delete().to(controller::delete_existing_user))
            .wrap(cors)
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}
