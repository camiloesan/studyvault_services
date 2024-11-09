use crate::channel;
use crate::channel::ChannelUpdateData;
use crate::sql_operations;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;

/// Returns all channels stored in database.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all channels.", body = [Channel]),
        (status = 500, description = "Internal server error occurred."),
    )
)]
#[get("/channels/all")]
pub async fn get_all_channels() -> impl Responder {
    let result = sql_operations::get_all_channels().await;

    match result {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            error!("Failed to fetch all channels: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Returns a ser of channels where a user is subscribed to.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all channels which the user is subscribed to.", body = [Channel]),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/subscriptions/user/{id}")]
pub async fn get_subscriptions_by_user(user_id: web::Path<u32>) -> impl Responder {
    let result = sql_operations::get_subscriptions_by_user(*user_id).await;

    match result {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            error!("Failed to fetch subscriptions by user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Returns all channels created by an user.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all channels created by an user.", body = [Channel]),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/channels/owner/{id}")]
pub async fn get_channels_created_by_user(user_id: web::Path<u32>) -> impl Responder {
    let result = sql_operations::get_channels_created_by_user(*user_id).await;

    match result {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            error!("Failed to fetch channels created by user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
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

pub async fn update_channel(
    channel_id: web::Path<u32>,
    channel_data: web::Json<ChannelUpdateData>,
) -> impl Responder {
    let name = channel_data.name.clone();
    let description = channel_data.description.clone();
    let category_id = channel_data.category_id;

    let result = sql_operations::update_channel(*channel_id, name, description, category_id).await;

    if !result {
        return HttpResponse::InternalServerError(); // 500
    }

    HttpResponse::Ok() // 200
}

pub async fn delete_channel(channel_id: web::Path<u32>) -> impl Responder {
    let id = *channel_id;

    let result = sql_operations::delete_channel(id).await;

    if !result {
        return HttpResponse::NotFound(); //404
    }

    HttpResponse::Ok() //200
}

pub async fn get_all_categories() -> impl Responder {
    let categories = sql_operations::get_all_categories().await;
    HttpResponse::Ok().json(categories)
}

pub async fn get_channel_name_by_id(path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    let channel_name = sql_operations::get_channel_name(channel_id).await;
    HttpResponse::Ok().json(channel_name)
}

pub async fn get_creator_id_by_channel_id(path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    let creator_id = sql_operations::get_creator_id(channel_id).await;
    HttpResponse::Ok().json(creator_id)
}
