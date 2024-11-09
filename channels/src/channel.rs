use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Channel {
    #[schema(example = "32", required = true)]
    pub creator_id: u32,
    #[schema(example = "Felix", required = true)]
    pub name: String,
    #[schema(example = "A channel for all things", required = true)]
    pub description: String,
    #[schema(example = "2", required = true)]
    pub category_id: u32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdateData {
    pub name: String,
    pub description: String,
    pub category_id: u32,
}
