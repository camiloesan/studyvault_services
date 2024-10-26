use crate::sql_operations;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_posts_by_channel(channel_id: web::Path<u32>) -> impl Responder {
    let posts = sql_operations::get_posts_by_channel(*channel_id).await;
    HttpResponse::Ok().json(posts)
}
