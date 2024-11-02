use crate::sql_operations;
use crate::user::{RegisterRequest, UserToUpdate};
use actix_web::{web, HttpResponse, Responder};

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

pub async fn update_existing_user(data: web::Json<UserToUpdate>) -> impl Responder {
    let request = sql_operations::update_user(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn delete_existing_user(data: web::Json<u32>) -> impl Responder {
    let request = sql_operations::delete_user(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn get_user_name_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user_name = sql_operations::get_user_name(user_id).await;
    HttpResponse::Ok().json(user_name)
}