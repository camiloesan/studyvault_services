use crate::model::Subscription;
use crate::repository::SubscriptionsRepository;
use async_trait::async_trait;
use mysql::{params, prelude::Queryable, Pool, PooledConn};

#[derive(Clone)]
pub struct MySQLSubscriptionsRepository {
    url: String,
}

impl MySQLSubscriptionsRepository {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    fn get_connection(&self) -> Result<PooledConn, mysql::Error> {
        let pool = Pool::new(self.url.as_str())?;
        let conn = pool.get_conn()?;
        Ok(conn)
    }
}

#[async_trait]
impl SubscriptionsRepository for MySQLSubscriptionsRepository {
    async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;
        let query =
            "INSERT INTO subscriptions (user_id, channel_id) VALUES (:user_id, :channel_id)";
        let result = conn.exec_iter(
            query,
            params! {
                "user_id" => subscription.user_id,
                "channel_id" => subscription.channel_id,
            },
        )?;

        Ok(result.affected_rows() == 1)
    }

    async fn unsubscribe(
        &self,
        subscription: Subscription,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;
        let query =
            "DELETE FROM subscriptions WHERE user_id = :user_id AND channel_id = :channel_id";
        let result = conn.exec_iter(
            query,
            params! {
                "user_id" => subscription.user_id,
                "channel_id" => subscription.channel_id,
            },
        )?;

        Ok(result.affected_rows() == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const URL: &str = "mysql://root:123456@127.0.0.1:6609/study_vault";

    #[tokio::test]
    async fn test_subscribe_to_channel_repo() {
        let repo = MySQLSubscriptionsRepository::new(URL);
        let subscription = Subscription {
            user_id: 2,
            channel_id: 1,
        };
        let result = repo.subscribe(subscription.clone()).await;
        let _ = repo.unsubscribe(subscription).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_unsubscribe_from_channel_repo() {
        let repo = MySQLSubscriptionsRepository::new(URL);
        let subscription = Subscription {
            user_id: 1,
            channel_id: 1,
        };
        let _ = repo.subscribe(subscription.clone()).await;
        let result = repo.unsubscribe(subscription).await;
        assert!(result.unwrap());
    }
}
