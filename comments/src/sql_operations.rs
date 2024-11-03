use crate::comment::{Comment, CommentToInsert, CommentToUpdate};
use data_access;
use mysql::{params, prelude::Queryable, Row, Value};

pub async fn comment(request: CommentToInsert) -> bool {
    let mut conn = data_access::get_connection();

    let query = "INSERT INTO comments (post_id, user_id, comment, publish_date, rating) VALUES (:post_id, :user_id, :comment, CURDATE(), :rating)";

    let result = conn
        .exec_iter(
            query,
            params! {
            "post_id" => request.post_id,
            "user_id" => request.user_id,
            "comment" => request.comment,
            "rating" => request.rating
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

pub async fn get_all_comments(post_id: u32) -> Vec<Comment> {
    let mut conn = data_access::get_connection();

    let query = "SELECT comment_id, post_id, user_id, comment, publish_date, rating FROM comments WHERE post_id = :post_id";

    let comments: Vec<Comment> = conn
        .exec_map(
            query,
            params! { "post_id" => post_id },
            |row: Row| {
                let publish_date_value: Value = row.get("publish_date").expect("Failed to get publish_date");
                let publish_date_str = match publish_date_value {
                    Value::Date(year, month, day, ..) => {
                        format!("{:04}-{:02}-{:02}", year, month, day)
                    },
                    _ => "1970-01-01".to_string(),
                };

                Comment {
                    comment_id: row.get("comment_id").unwrap_or_default(),
                    post_id: row.get("post_id").unwrap_or_default(),
                    user_id: row.get("user_id").unwrap_or_default(),
                    comment: row.get("comment").unwrap_or_default(),
                    publish_date: publish_date_str,
                    rating: row.get("rating").unwrap_or_default(),
                }
            },
        )
        .expect("Failed to execute select query and map results");

    comments
}

pub async fn update_comment(request: CommentToUpdate) -> bool {
    let mut conn = data_access::get_connection();

    let query = "UPDATE comments SET comment = :comment, publish_date = CURDATE(), rating = :rating WHERE comment_id = :comment_id";

    let result = conn
        .exec_iter(
            query,
            params! {
            "comment_id" => request.comment_id,
            "comment" => request.comment,
            "rating" => request.rating,
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

pub async fn delete_comment(id: u32) -> bool {
    let mut conn = data_access::get_connection();

    let query = "DELETE FROM comments WHERE comment_id = :comment_id";

    let result = conn
        .exec_iter(
            query,
            params! {
            "comment_id" => id
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}