use crate::model::Subscription;
use async_trait::async_trait;

#[async_trait]
pub trait SubscriptionsRepository {
    async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<bool, Box<dyn std::error::Error>>;
    async fn unsubscribe(
        &self,
        subscription: Subscription,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
