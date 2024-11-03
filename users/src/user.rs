use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserToUpdate {
    pub id: u32,
    pub name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize)]
pub struct VerificationRequest {
    pub email: String,
    pub code: String,
}