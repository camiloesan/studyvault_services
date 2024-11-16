use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VerificationRequest {
    pub email: String,
    pub code: String,
}