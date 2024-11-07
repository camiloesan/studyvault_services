use crate::sql_operations;
use crate::subscription::Subscription;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_subscription(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = sql_operations::subscribe_to_channel(user_id, channel_id).await;

    match result {
        Ok(true) => HttpResponse::Ok(),                   //200
        Ok(false) => HttpResponse::InternalServerError(), //500
        Err(_) => HttpResponse::InternalServerError(),    //500
    }
}

pub async fn unsubscribe_from_channel(subscription: web::Json<Subscription>) -> impl Responder {
    let user_id = subscription.user_id;
    let channel_id = subscription.channel_id;

    let result = sql_operations::unsubscribe_from_channel(user_id, channel_id).await;

    match result {
        Ok(true) => HttpResponse::Ok(),                   //200
        Ok(false) => HttpResponse::InternalServerError(), //500
        Err(_) => HttpResponse::InternalServerError(),    //500
    }
}
