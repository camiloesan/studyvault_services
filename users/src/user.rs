use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserToUpdate {
    pub id: u32,
    pub name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserName {
    pub name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PasswordToUpdate {
    pub email: String,
    pub password: String,
}