use crate::sql_operations;
use crate::auth::{LoginData, VerificationRequest};
use actix_web::{web, HttpResponse, Responder};
use crate::email_operations::{generate_verification_code, send_verification_email};
use crate::email_operations::VERIFICATION_CODES;
use auth::generate_jwt;
use actix_web::post;

/// Logs in a user and returns a JWT token if successful.
#[utoipa::path(
    request_body = LoginData,
    responses(
        (status = 200, description = "User logged in successfully.", body = User, headers(
            ("x-token" = String, description = "JWT token for authenticated requests.")
        )),
        (status = 401, description = "Unauthorized. Invalid credentials."),
        (status = 500, description = "Internal server error.")
    )
)]
#[post("/login")]
pub async fn login_user(login_data: web::Json<LoginData>) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    let result = sql_operations::login(email, password).await;

    if let Some(user) = result {
        match generate_jwt(user.user_id as i32) {
            Ok(token) => {
                HttpResponse::Ok()
                    .insert_header(("x-token", token))
                    .json(user)
            },
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

/// Generates a verification code and sends it to the specified email.
#[utoipa::path(
    request_body = email,
    responses(
        (status = 200, description = "Verification code sent successfully."),
        (status = 500, description = "Internal server error.")
    )
)]
#[post("/user/verification/request")]
pub async fn request_verification(email: web::Json<String>) -> impl Responder {
    let code = generate_verification_code();
    
    send_verification_email(email.clone(), code.clone()).await;

    VERIFICATION_CODES.lock().unwrap().insert(email.clone(), code);

    HttpResponse::Ok().finish()
}

/// Verifies the code sent to the user's email.
#[utoipa::path(
    request_body = VerificationRequest,
    responses(
        (status = 200, description = "Verification successful."),
        (status = 401, description = "Unauthorized. Invalid or expired code.")
    )
)]
#[post("/user/verify")]
pub async fn verify_code(data: web::Json<VerificationRequest>) -> impl Responder {
    let VerificationRequest { email, code } = data.into_inner();

    let mut codes = VERIFICATION_CODES.lock().unwrap();
    
    if let Some(stored_code) = codes.get(&email) {
        if stored_code == &code {
            codes.remove(&email);
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}