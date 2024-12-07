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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_success() {
        let email = "lizrm@uv.mx".to_string();
        let password = "123456".to_string();
        let result = login(email, password).await;

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(user.email, "juan@uv.mx");
        assert!(!user.name.is_empty());
        assert!(!user.last_name.is_empty());
        assert!(user.user_id > 0);
        assert!(user.user_type_id > 0);
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let email = "juan@uv.mx".to_string();
        let password = "123456".to_string();
        let result = login(email, password).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_login_invalid_email() {
        let email = "margaritagh@uv.mx".to_string();
        let password = "123456".to_string();
        let result = login(email, password).await;

        assert!(result.is_none());
    }
}
