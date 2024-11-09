use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Subscription {
    #[schema(example = "1", required = true)]
    pub user_id: u32,
    #[schema(example = "13", required = true)]
    pub channel_id: u32,
}
