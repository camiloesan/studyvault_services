use data_access;
use mysql::{params, prelude::Queryable};

/// Subscribes an user to a channel in mysql database.
pub async fn subscribe_to_channel(user_id: u32, channel_id: u32) -> Result<bool, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query = "INSERT INTO subscriptions (user_id, channel_id) VALUES (:user_id, :channel_id)";
    let result = conn.exec_iter(
        &query,
        params! {
            "user_id" => user_id,
            "channel_id" => channel_id,
        },
    )?;

    Ok(result.affected_rows() == 1)
}

/// Unsubscribes an user from a channel in mysql database.
pub async fn unsubscribe_from_channel(user_id: u32, channel_id: u32) -> Result<bool, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query = "DELETE FROM subscriptions WHERE user_id = :user_id AND channel_id = :channel_id";
    let result = conn.exec_iter(
        &query,
        params! {
            "user_id" => user_id,
            "channel_id" => channel_id,
        },
    )?;

    Ok(result.affected_rows() == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_unsubscribe_from_channel() {
        // create user
        // create channel
        let _ = subscribe_to_channel(1, 1).await;
        let result = unsubscribe_from_channel(1, 1).await;

        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_subscribe_to_channel() {
        let result = subscribe_to_channel(2, 1).await;
        let _ = unsubscribe_from_channel(2, 1).await;

        assert!(result.unwrap());
    }
}
