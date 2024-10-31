use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub post_id: u32,
    pub user_id: u32,
    pub comment: String,
    pub publish_date: String,
    pub rating: u32
}