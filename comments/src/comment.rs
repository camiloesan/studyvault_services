use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct CommentToInsert {
    pub post_id: u32,
    pub user_id: u32,
    pub comment: String,
    pub rating: u32
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub comment_id: u32,
    pub post_id: u32,
    pub user_id: u32,
    pub comment: String,
    pub publish_date: String,
    pub rating: u32
}

#[derive(Serialize, Deserialize)]

pub struct CommentToUpdate {
    pub comment_id: u32,
    pub comment: String,
    pub rating: u32
}