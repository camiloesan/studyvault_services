use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]

pub struct CommentToInsert {
    pub post_id: u32,
    pub user_id: u32,
    pub comment: String,
    pub rating: u32
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Comment {
    pub comment_id: u32,
    pub post_id: u32,
    pub user_id: u32,
    pub comment: String,
    pub publish_date: String,
    pub rating: u32
}

#[derive(Serialize, Deserialize, ToSchema)]

pub struct CommentToUpdate {
    pub comment_id: u32,
    pub comment: String,
    pub rating: u32
}