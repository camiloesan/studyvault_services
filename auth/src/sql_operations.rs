use data_access;
use mysql::{params, prelude::Queryable, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: u32,
    pub user_type_id: u32,
    pub name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn login(email: String, password: String) -> Option<User> {
    let mut conn = data_access::get_connection_safe().ok()?;

    let query = "
        SELECT user_id, user_type_id, name, last_name, email, password
        FROM users
        WHERE email = :email AND password = :password";

        let row: Option<Row> = conn
        .exec_first(query, params! { "email" => email, "password" => password })
        .ok()?;

    if let Some(mut row) = row {
        let user_id: u32 = row.take("user_id").unwrap();
        let user_type_id: u32 = row.take("user_type_id").unwrap();
        let name: String = row.take("name").unwrap();
        let last_name: String = row.take("last_name").unwrap();
        let email: String = row.take("email").unwrap();

        return Some(User {
            user_id,
            user_type_id,
            name,
            last_name,
            email,
        });
    }
    
    None
}