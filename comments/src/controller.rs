use crate::{comment::CommentToUpdate, sql_operations};
use crate::comment::CommentToInsert;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;

/// Create a new comment.
#[utoipa::path(
    request_body = CommentToInsert,
    responses(
        (status = 200, description = "Comment created successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[post("/comment")]
pub async fn comment_post(data: web::Json<CommentToInsert>) -> impl Responder {
    match sql_operations::comment(data.into_inner()).await {
        true => HttpResponse::Ok().json("Comment created successfully."),
        false => {
            error!("Failed to create comment");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Retrieve all comments by post ID.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all comments for the post.", body = [Comment]),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/comment/all/{id}")]
pub async fn get_all_comments_by_post_id(path: web::Path<u32>) -> impl Responder {
    let post_id = path.into_inner();
    let comments = sql_operations::get_all_comments(post_id).await;
    HttpResponse::Ok().json(comments)
}

/// Update an existing comment.
#[utoipa::path(
    request_body = CommentToUpdate,
    responses(
        (status = 200, description = "Comment updated successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[put("/comment/update/{id}")]
pub async fn update_existing_comment(data: web::Json<CommentToUpdate>) -> impl Responder {
    match sql_operations::update_comment(data.into_inner()).await {
        true => HttpResponse::Ok().json("Comment updated successfully."),
        false => {
            error!("Failed to update comment");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete an existing comment by ID.
#[utoipa::path(
    responses(
        (status = 200, description = "Comment deleted successfully."),
        (status = 404, description = "Comment not found."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[delete("/comment/delete/{id}")]
pub async fn delete_existing_comment(path: web::Path<u32>) -> impl Responder {
    let comment_id = path.into_inner();
    match sql_operations::delete_comment(comment_id).await {
        true => HttpResponse::Ok().json("Comment deleted successfully."),
        false => {
            error!("Failed to delete comment");
            HttpResponse::InternalServerError().finish()
        }
    }
}
