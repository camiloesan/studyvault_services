use data_access;
use mysql::{params, prelude::Queryable, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Channel {
    channel_id: u32,
    creator_id: u32,
    creator_name: String,
    creator_last_name: String,
    name: String,
    description: String,
    category_name: String,
}

pub async fn get_all_channels() -> Result<Vec<Channel>, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query =
        "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name, categories.name as category_name
        FROM channels INNER JOIN users ON channels.creator_id = users.user_id
        INNER JOIN categories ON channels.category_id = categories.category_id";
    let mut channels: Vec<Channel> = Vec::new();
    conn.query_map(&query, |mut row: Row| {
        let channel = Channel {
            channel_id: row.take("channel_id").unwrap(),
            creator_id: row.take("creator_id").unwrap(),
            creator_name: row.take("creator_name").unwrap(),
            creator_last_name: row.take("creator_last_name").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
            category_name: row.take("category_name").unwrap(),
        };

        channels.push(channel);
    })?;

    Ok(channels)
}

pub async fn get_subscriptions_by_user(user_id: u32) -> Result<Vec<Channel>, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query =
        "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name, categories.name as category_name
        FROM channels INNER JOIN users ON channels.creator_id = users.user_id
        INNER JOIN categories ON channels.category_id = categories.category_id
        WHERE channels.channel_id IN (SELECT channel_id FROM subscriptions WHERE user_id = :user_id)";
    let mut channels: Vec<Channel> = Vec::new();

    conn.exec_map(&query, params! { "user_id" => user_id }, |mut row: Row| {
        let channel = Channel {
            channel_id: row.take("channel_id").unwrap(),
            creator_id: row.take("creator_id").unwrap(),
            creator_name: row.take("creator_name").unwrap(),
            creator_last_name: row.take("creator_last_name").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
            category_name: row.take("category_name").unwrap(),
        };
        channels.push(channel);
    })?;

    Ok(channels)
}

pub async fn get_channels_created_by_user(user_id: u32) -> Result<Vec<Channel>, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query =
        "SELECT channels.*, users.name as creator_name, users.last_name as creator_last_name, categories.name as category_name
        FROM channels INNER JOIN users ON channels.creator_id = users.user_id
        INNER JOIN categories ON channels.category_id = categories.category_id
        WHERE channels.creator_id = :creator_id";

    let mut channels: Vec<Channel> = Vec::new();
    conn.exec_map(
        &query,
        params! { "creator_id" => user_id },
        |mut row: Row| {
            let channel = Channel {
                channel_id: row.take("channel_id").unwrap(),
                creator_id: row.take("creator_id").unwrap(),
                creator_name: row.take("creator_name").unwrap(),
                creator_last_name: row.take("creator_last_name").unwrap(),
                name: row.take("name").unwrap(),
                description: row.take("description").unwrap(),
                category_name: row.take("category_name").unwrap(),
            };
            channels.push(channel);
        },
    )?;

    Ok(channels)
}

pub async fn create_channel(
    creator_id: u32,
    name: String,
    description: String,
    category_id: u32,
) -> bool {
    let mut conn = data_access::get_connection();
    let query = "INSERT INTO channels (creator_id, name, description, category_id)
        VALUES (:creator_id, :name, :description, :category_id)";

    let result = conn
        .exec_iter(
            query,
            params! {
                "creator_id" => creator_id,
                "name" => name,
                "description" => description,
                "category_id" => category_id,
            },
        )
        .expect("Failed to create channel")
        .affected_rows();

    result == 1
}

pub async fn update_channel(
    channel_id: u32,
    name: String,
    description: String,
    category_id: u32,
) -> bool {
    let mut conn = data_access::get_connection();
    let query =
        "UPDATE channels SET name = :name, description = :description, category_id = :category_id
        WHERE channel_id = :channel_id";

    let result = conn
        .exec_iter(
            query,
            params! {
                "name" => name,
                "description" => description,
                "category_id" => category_id,
                "channel_id" => channel_id,
            },
        )
        .expect("Failed to update channel")
        .affected_rows();

    result == 1
}

pub async fn delete_channel(channel_id: u32) -> bool {
    let mut conn = data_access::get_connection();

    let query = "DELETE FROM channels WHERE channel_id = :channel_id";

    let result = conn
        .exec_iter(
            query,
            params! {
                "channel_id" => channel_id
            },
        )
        .expect("Failed to execute delete channel query")
        .affected_rows();

    result == 1
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    category_id: u32,
    name: String,
}

pub async fn get_all_categories() -> Vec<Category> {
    let mut conn = data_access::get_connection();
    let query = "SELECT category_id, name FROM categories";
    let mut categories: Vec<Category> = Vec::new();

    conn.query_map(&query, |mut row: Row| {
        let category = Category {
            category_id: row.take("category_id").unwrap(),
            name: row.take("name").unwrap(),
        };
        categories.push(category);
    })
    .expect("Failed to fetch categories information");

    categories
}

pub async fn get_channel_name(channel_id: u32) -> String {
    let mut conn = data_access::get_connection();

    let query = "SELECT name FROM channels WHERE channel_id = :channel_id";

    let result: Vec<String> = conn
        .exec(
            query,
            params! {
                "channel_id" => channel_id
            },
        )
        .expect("Failed to execute query");

    result
        .into_iter()
        .next()
        .unwrap_or_else(|| "Unknown Channel".to_string())
}

pub async fn get_creator_id(channel_id: u32) -> Option<u32> {
    let mut conn = data_access::get_connection();

    let query = "SELECT creator_id FROM channels WHERE channel_id = :channel_id";

    let result: Vec<u32> = conn
        .exec(
            query,
            params! {
                "channel_id" => channel_id
            },
        )
        .expect("Failed to execute query");

    result.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_channels() {
        let result = get_all_channels().await;
        let channels = result.unwrap();
        assert!(channels.is_empty() == false);
    }

    #[tokio::test]
    async fn test_get_channels_by_user_exists() {
        // do pre and post
        let result = get_channels_created_by_user(2).await;
        let channels = result.unwrap();
        println!("Channels count: {}", channels.len());
        assert!(channels.is_empty() == false);
    }

    #[tokio::test]
    async fn test_get_channels_by_user_not_exists() {
        // do pre and post
        let result = get_channels_created_by_user(100).await;
        let channels = result.unwrap();
        assert!(channels.is_empty() == true);
    }
}
