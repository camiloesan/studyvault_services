use crate::{comment::CommentToUpdate, sql_operations};
use crate::comment::CommentToInsert;
use actix_web::{web, HttpResponse, Responder};

pub async fn comment_post(data: web::Json<CommentToInsert>) -> impl Responder {
    let request = sql_operations::comment(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn get_all_comments_by_post_id(path: web::Path<u32>) -> impl Responder {
    let post_id = path.into_inner();
    let comments = sql_operations::get_all_comments(post_id).await;
    HttpResponse::Ok().json(comments)
}

pub async fn update_existing_comment(data: web::Json<CommentToUpdate>) -> impl Responder {
    let request = sql_operations::update_comment(data.into_inner()).await;

    if !request {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

pub async fn delete_existing_comment(path: web::Path<u32>) -> impl Responder {
    let comment_id = path.into_inner();
    let comments = sql_operations::delete_comment(comment_id).await;
    HttpResponse::Ok().json(comments)
}