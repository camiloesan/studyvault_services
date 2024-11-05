use crate::sql_operations;
use crate::auth::{LoginData, VerificationRequest};
use actix_web::{web, HttpResponse, Responder};
use crate::email_operations::{generate_verification_code, send_verification_email};
use crate::email_operations::VERIFICATION_CODES;

pub async fn login_user(login_data: web::Json<LoginData>) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    let result = sql_operations::login(email, password).await;

    if let Some(user) = result {
        return HttpResponse::Ok().json(user); //200
    }

    HttpResponse::Unauthorized().finish() //401
}

pub async fn request_verification(email: web::Json<String>) -> impl Responder {
    let code = generate_verification_code();
    
    send_verification_email(email.clone(), code.clone()).await;

    VERIFICATION_CODES.lock().unwrap().insert(email.clone(), code);

    HttpResponse::Ok().finish()
}

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