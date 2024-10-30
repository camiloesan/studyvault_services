use crate::post::Post;
use actix_web::cookie::time::Date;
use data_access;
use mysql::{params, prelude::Queryable, Row};

pub async fn get_posts_by_channel_id(channel_id: u32) -> Vec<Post> {
    let mut conn = data_access::get_connection();
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
    )
    .expect("failed to get developer information");

    posts
}

pub async fn create_post(
    uuid: String,
    channel_id: u32,
    file_name: String,
    title: String,
    description: String,
) -> bool {
    let mut conn = data_access::get_connection();

    let first_query = "INSERT INTO files (file_id, name) VALUES (:file_id, :file_name)";
    let f_result = conn
        .exec_iter(
            first_query,
            params! {
                "file_id" => &uuid,
                "file_name" => file_name,
            },
        )
        .expect("Failed to insert file")
        .affected_rows();

    let second_query = "INSERT INTO posts (channel_id, file_id, title, description, publish_date)
        VALUES (:channel_id, :file_id, :title, :description, NOW())";
    let s_result = conn
        .exec_iter(
            second_query,
            params! {
                "channel_id" => channel_id,
                "file_id" => uuid,
                "title" => title,
                "description" => description,
            },
        )
        .expect("Failed to create post")
        .affected_rows();

    f_result == 1 && s_result == 1
}

pub async fn _delete_post_by_file_uuid(uuid: String) -> bool {
    let mut conn = data_access::get_connection();

    let query = "DELETE FROM posts WHERE file_id = :file_id";
    let result = conn
        .exec_iter(
            query,
            params! {
                "file_id" => uuid,
            },
        )
        .expect("Failed to delete post")
        .affected_rows();

    result == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_posts_by_channel() {
        let posts = get_posts_by_channel_id(1).await;
        assert_eq!(posts.len(), 2);
    }

    #[tokio::test]
    async fn test_create_post() {
        let channel_id = 1;
        let file_name = "test.pdf".to_string();
        let title = "Test Post".to_string();
        let description = "This is a test post".to_string();
        let uuid = Uuid::new_v4().to_string();

        let result = create_post(uuid.clone(), channel_id, file_name, title, description).await;

        _delete_post_by_file_uuid(uuid).await;

        assert_eq!(result, true);
    }
}
