use crate::sql_operations;
use crate::user::{RegisterRequest, UserToUpdate, PasswordToUpdate};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;

/// Register a new user.
#[utoipa::path(
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[post("/register")]
pub async fn register_new_user(data: web::Json<RegisterRequest>) -> impl Responder {
    match sql_operations::register_user(data.into_inner()).await {
        true => HttpResponse::Ok().json("User registered successfully."),
        false => {
            error!("Failed to register user");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Retrieve all user emails.
#[utoipa::path(
    responses(
        (status = 200, description = "List of user emails retrieved successfully.", body = [String]),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/user/email/all")]
pub async fn get_all_emails() -> impl Responder {
    let emails = sql_operations::get_all_user_emails().await;
    HttpResponse::Ok().json(emails)
}

/// Update an existing user.
#[utoipa::path(
    request_body = UserToUpdate,
    responses(
        (status = 200, description = "User updated successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[put("/update/{id}")]
pub async fn update_existing_user(data: web::Json<UserToUpdate>) -> impl Responder {
    match sql_operations::update_user(data.into_inner()).await {
        true => HttpResponse::Ok().json("User updated successfully."),
        false => {
            error!("Failed to update user");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete an existing user by ID.
#[utoipa::path(
    responses(
        (status = 200, description = "User deleted successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[delete("/delete/{id}")]
pub async fn delete_existing_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_operations::delete_user(user_id).await {
        true => HttpResponse::Ok().json("User deleted successfully."),
        false => {
            error!("Failed to delete user with ID: {}", user_id);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Retrieve a user's name by ID.
#[utoipa::path(
    responses(
        (status = 200, description = "User name retrieved successfully.", body = String),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[get("/user/name/{id}")]
pub async fn get_user_name_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user_name = sql_operations::get_user_name(user_id).await;
    HttpResponse::Ok().json(user_name)
}

/// Update a user's password.
#[utoipa::path(
    request_body = PasswordToUpdate,
    responses(
        (status = 200, description = "Password updated successfully."),
        (status = 500, description = "Internal server error occurred.")
    )
)]
#[put("/password/update")]
pub async fn update_user_password(data: web::Json<PasswordToUpdate>) -> impl Responder {
    match sql_operations::update_password(data.into_inner()).await {
        true => HttpResponse::Ok().json("Password updated successfully."),
        false => {
            error!("Failed to update password");
            HttpResponse::InternalServerError().finish()
        }
    }
}
