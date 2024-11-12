use std::result;

use crate::post::Post;
use actix_web::cookie::time::Date;
use data_access;
use mysql::{params, prelude::Queryable, Row};

pub async fn get_posts_by_channel_id(channel_id: u32) -> Result<Vec<Post>, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;
    let query = "SELECT * FROM posts WHERE channel_id = :channel_id";

    let mut posts: Vec<Post> = Vec::new();

    conn.exec_map(
        &query,
        params! { "channel_id" => channel_id },
        |mut row: Row| {
            let pdate: Date = row.take("publish_date").unwrap();
            let pdate_str = pdate.to_string();

            let post = Post {
                post_id: row.take("post_id").unwrap(),
                channel_id: row.take("channel_id").unwrap(),
                file_id: row.take("file_id").unwrap(),
                title: row.take("title").unwrap(),
                description: row.take("description").unwrap(),
                publish_date: pdate_str,
            };
            posts.push(post);
        },
    )?;

    Ok(posts)
}

pub async fn create_post(
    uuid: String,
    channel_id: u32,
    file_name: String,
    title: String,
    description: String,
) -> Result<bool, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;

    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let first_query = "INSERT INTO files (file_id, name) VALUES (:file_id, :file_name)";
    transaction.exec_iter(
        first_query,
        params! {
            "file_id" => &uuid,
            "file_name" => file_name,
        },
    )?;

    let second_query = "INSERT INTO posts (channel_id, file_id, title, description, publish_date)
        VALUES (:channel_id, :file_id, :title, :description, NOW())";
    transaction.exec_iter(
        second_query,
        params! {
            "channel_id" => channel_id,
            "file_id" => uuid,
            "title" => title,
            "description" => description,
        },
    )?;

    let affected_rows = transaction.affected_rows();
    let mut result = false;
    if affected_rows == 1 {
        transaction.commit()?;
        result = true;
    } else {
        transaction.rollback()?;
    }

    Ok(result)
}

pub async fn _delete_post_by_file_uuid(uuid: String) -> Result<bool, mysql::Error> {
    let mut conn = data_access::get_connection_safe()?;

    let mut transaction = conn.start_transaction(mysql::TxOpts::default())?;
    let query = "DELETE FROM files WHERE file_id = :file_id";
    transaction.exec_iter(
        query,
        params! {
            "file_id" => &uuid,
        },
    )?;
    let affected_rows = transaction.affected_rows();

    let mut result = false;
    if affected_rows == 1 {
        transaction.commit()?;
        result = true;
    } else {
        transaction.rollback()?;
    }

    Ok(result)
}

pub async fn get_file_name(uuid: String) -> Result<String, mysql::Error> {
    let mut conn = data_access::get_connection();
    let query = "SELECT name FROM files WHERE file_id = :file_id";

    let mut result: String = Default::default();

    conn.exec_map(&query, params! { "file_id" => uuid }, |mut row: Row| {
        result = row.take("name").unwrap()
    })?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_posts_by_channel() {
        // pre
        let channel_id = 1;
        let file_name = "test.pdf".to_string();
        let title = "Test Post".to_string();
        let description = "This is a test post".to_string();
        let uuid = Uuid::new_v4().to_string();
        let _ = create_post(uuid.clone(), channel_id, file_name, title, description).await;

        let result = get_posts_by_channel_id(1).await;
        let posts = result.unwrap();
        assert_eq!(posts.len() > 0, true);

        // post
        let _ = _delete_post_by_file_uuid(uuid).await;
    }

    #[tokio::test]
    async fn test_create_post() {
        let channel_id = 1;
        let file_name = "test.pdf".to_string();
        let title = "Test Post".to_string();
        let description = "This is a test post".to_string();
        let uuid = Uuid::new_v4().to_string();
        let result = create_post(uuid.clone(), channel_id, file_name, title, description).await;

        assert_eq!(result.unwrap(), true);

        // post
        let _ = _delete_post_by_file_uuid(uuid).await;
    }
}
