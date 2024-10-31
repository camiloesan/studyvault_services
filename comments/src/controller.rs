use crate::sql_operations;
use crate::comment::Comment;
use actix_web::{web, HttpResponse, Responder};

pub async fn comment_post(data: web::Json<Comment>) -> impl Responder {
    let request = sql_operations::comment(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn get_all_comments_by_post_id(data: web::Json<u32>) -> impl Responder {
    let comments = sql_operations::get_all_comments(data.into_inner()).await;
    HttpResponse::Ok().json(comments)
}