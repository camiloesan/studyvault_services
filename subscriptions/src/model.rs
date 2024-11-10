use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Subscription {
    #[schema(example = "2", required = true)]
    pub user_id: u32,
    #[schema(example = "13", required = true)]
    pub channel_id: u32,
}
