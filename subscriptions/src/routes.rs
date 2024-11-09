use crate::sql_operations;
use crate::subscription::Subscription;
use actix_web::{delete, post, web, HttpResponse, Responder};

/// Subscribe an user to a channel given Subscription schema.
#[utoipa::path(
    request_body = Subscription,
    responses(
        (status = 200, description = "Subscribes an user to a channel."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[post("/subscription")]
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

/// Unsubscribes an user from a channel given the Subscription schema.
#[utoipa::path(
    request_body = Subscription,
    responses(
        (status = 200, description = "Unsubscribes an user from a channel."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[delete("/unsubscribe")]
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
