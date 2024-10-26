use crate::channel;
use crate::sql_operations;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_all_channels() -> impl Responder {
    let channels = sql_operations::get_all_channels().await;
    HttpResponse::Ok().json(channels)
}

pub async fn get_subscriptions_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = sql_operations::get_subscriptions_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

pub async fn get_channels_created_by_user(user_id: web::Path<u32>) -> impl Responder {
    let channels = sql_operations::get_channels_created_by_user(*user_id).await;
    HttpResponse::Ok().json(channels)
}

pub async fn create_channel(channel: web::Json<channel::Channel>) -> impl Responder {
    let creator_id = channel.creator_id;
    let name = channel.name.clone();
    let description = channel.description.clone();
    let category_id = channel.category_id;

    let result = sql_operations::create_channel(creator_id, name, description, category_id).await;

    if !result {
        return HttpResponse::InternalServerError(); //500
    }

    HttpResponse::Ok() //200
}

pub async fn get_all_categories() -> impl Responder {
    let categories = sql_operations::get_all_categories().await;
    HttpResponse::Ok().json(categories)
}
