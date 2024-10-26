use crate::sql_operations;
use crate::user::LoginData;
use crate::user::RegisterRequest;
use actix_web::{web, HttpResponse, Responder};

pub async fn login_user(login_data: web::Json<LoginData>) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    let result = sql_operations::login(email, password).await;

    if let Some(user) = result {
        return HttpResponse::Ok().json(user); //200
    }

    HttpResponse::Unauthorized().finish() //401
}

pub async fn register_new_user(data: web::Json<RegisterRequest>) -> impl Responder {
    let request = sql_operations::register_user(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn get_all_emails() -> impl Responder {
    let emails = sql_operations::get_all_user_emails().await;
    HttpResponse::Ok().json(emails)
}
