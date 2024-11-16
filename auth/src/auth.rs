use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginData {
    #[schema(example = "zS21013882@estudiantes.uv.mx", required = true)]
    pub email: String,
    #[schema(example = 
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8", 
        required = true)]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VerificationRequest {
    pub email: String,
    pub code: String,
}