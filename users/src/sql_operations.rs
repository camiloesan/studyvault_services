use crate::user::{RegisterRequest, UserName, UserToUpdate};
use data_access;
use mysql::{params, prelude::Queryable, Row, from_row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: u32,
    pub user_type_id: u32,
    pub name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn register_user(request: RegisterRequest) -> bool {
    let user_type_id = if request.email.ends_with("@estudiantes.uv.mx") {
        2
    } else if request.email.ends_with("@uv.mx") {
        1
    } else {
        return false;
    };

    let mut conn = data_access::get_connection();

    let query = "INSERT INTO users (user_type_id, name, last_name, email, password) VALUES (:user_type_id, :name, :last_name, :email, :password)";

    let result = conn
        .exec_iter(
            query,
            params! {
            "user_type_id" => user_type_id,
            "name" => request.name,
            "last_name" => request.last_name,
            "email" => request.email,
            "password" => request.password,
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

pub async fn get_all_user_emails() -> Vec<String> {
    let mut conn = data_access::get_connection();
    let query = "SELECT email FROM users";
    let mut emails: Vec<String> = Vec::new();

    conn.query_map(&query, |mut row: Row| {
        let email: String = row.take("email").unwrap();
        emails.push(email);
    })
    .expect("failed to get user emails");

    emails
}

pub async fn update_user(request: UserToUpdate) -> bool {
    let mut conn = data_access::get_connection();

    let query = "UPDATE users SET name = :name, last_name = :last_name WHERE user_id = :user_id";

    let result = conn
        .exec_iter(
            query,
            params! {
            "user_id" => request.id,
            "name" => request.name,
            "last_name" => request.last_name,
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

pub async fn delete_user(id: u32) -> bool {
    let mut conn = data_access::get_connection();

    let query = "DELETE FROM users WHERE user_id = :user_id";

    let result = conn
        .exec_iter(
            query,
            params! {
            "user_id" => id
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

pub async fn get_user_name(user_id: u32) -> UserName {
    let mut conn = data_access::get_connection();

    let query = "SELECT name, last_name FROM users WHERE user_id = :user_id";

    let result = conn
        .exec_iter(query, params! {
            "user_id" => user_id
        })
        .expect("Failed to execute query");

    for row in result {
        let (name, last_name): (String, String) = from_row::<(String, String)>(row.expect("Row error"));
        return UserName { name, last_name };
    }

    UserName {
        name: "Unknown".to_string(),
        last_name: "User".to_string(),
    }
}