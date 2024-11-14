use std::result;

use crate::channel;
use crate::channel::ChannelUpdateData;
use crate::sql_operations;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
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

/// Create a channel given the Channel schema.
#[utoipa::path(
    request_body = Channel,
    responses(
        (status = 200, description = "Channel created succesfully.", body = Channel),
        (status = 500, description = "Internal server error ocurred."),
    )
)]
#[post("/channel/create")]
pub async fn create_channel(channel: web::Json<channel::Channel>) -> impl Responder {
    let result = sql_operations::create_channel(
        channel.creator_id,
        channel.name.clone(),
        channel.description.clone(),
        channel.category_id,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Channel created successfully."),
        Err(e) => {
            error!("Failed to create channel: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Update a channel given the ChannelUpdateData schema.
#[utoipa::path(
    request_body = ChannelUpdateData,
    responses(
        (status = 200, description = "Channel updated successfully.", body = ChannelUpdateData),
        (status = 500, description = "Internal server error ocurred."),
    )
)]
#[put("/channel/update/{id}")]
pub async fn update_channel(
    channel_id: web::Path<u32>,
    channel_data: web::Json<ChannelUpdateData>,
) -> impl Responder {
    let result = sql_operations::update_channel(
        *channel_id,
        channel_data.name.clone(),
        channel_data.description.clone(),
        channel_data.category_id,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Channel updated successfully."),
        Err(e) => {
            error!("Failed to update channel: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete a channel by ID.
#[utoipa::path(
    responses(
        (status = 200, description = "Channel deleted successfully."),
        (status = 404, description = "Channel not found."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[delete("/channel/delete/{id}")]
pub async fn delete_channel(channel_id: web::Path<u32>) -> impl Responder {
    let result = sql_operations::delete_channel(*channel_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Channel deleted successfully."),
        Err(e) => {
            error!("Failed to delete channel: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Returns all channel categories.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all channel categories.", body = [Category]),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/categories/all")]
pub async fn get_all_categories() -> impl Responder {
    let result = sql_operations::get_all_categories().await;

    match result {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            error!("Failed to fetch all categories: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Returns the name of a channel by ID.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns the channel name."),
        (status = 404, description = "Channel not found."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/channel/name/{id}")]
pub async fn get_channel_name_by_id(path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    let channel_name = sql_operations::get_channel_name(channel_id).await;
    HttpResponse::Ok().json(channel_name)
}

/// Returns the creator ID of a channel by channel ID.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns the creator ID."),
        (status = 404, description = "Channel not found."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/creator/channel/{id}")]
pub async fn get_creator_id_by_channel_id(path: web::Path<u32>) -> impl Responder {
    let channel_id = path.into_inner();
    let creator_id = sql_operations::get_creator_id(channel_id).await;
    HttpResponse::Ok().json(creator_id)
}
