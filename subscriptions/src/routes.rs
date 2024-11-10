use crate::model::Subscription;
use crate::{repository::SubscriptionsRepository, sql_repo::MySQLSubscriptionsRepository};
use actix_web::{delete, post, web, HttpResponse, Responder};
use log::error;

/// Subscribe an user to a channel given Subscription schema.
#[utoipa::path(
    request_body = Subscription,
    responses(
        (status = 200, description = "Subscribes an user to a channel."),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[post("/subscription")]
pub async fn create_subscription(
    repo: web::Data<MySQLSubscriptionsRepository>,
    subscription: web::Json<Subscription>,
) -> impl Responder {
    match repo.subscribe(subscription.into_inner()).await {
        Ok(true) => HttpResponse::Ok(),                   //200
        Ok(false) => HttpResponse::InternalServerError(), //500
        Err(e) => {
            error!("Database exception ocurred: {}", e);
            HttpResponse::InternalServerError()
        } //500
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
pub async fn unsubscribe_from_channel(
    repo: web::Data<MySQLSubscriptionsRepository>,
    subscription: web::Json<Subscription>,
) -> impl Responder {
    match repo.unsubscribe(subscription.into_inner()).await {
        Ok(true) => HttpResponse::Ok(),                   //200
        Ok(false) => HttpResponse::InternalServerError(), //500
        Err(e) => {
            error!("Database exception ocurred: {}", e);
            HttpResponse::InternalServerError()
        } //500
    }
}
