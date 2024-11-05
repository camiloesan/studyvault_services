use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationRequest {
    pub email: String,
    pub code: String,
}