use crate::user::{PasswordToUpdate, RegisterRequest, UserName, UserToUpdate};
use data_access;
use mysql::{from_row, params, prelude::Queryable, Row};

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
        .exec_iter(
            query,
            params! {
                "user_id" => user_id
            },
        )
        .expect("Failed to execute query");

    for row in result {
        let (name, last_name): (String, String) =
            from_row::<(String, String)>(row.expect("Row error"));
        return UserName { name, last_name };
    }

    UserName {
        name: "Unknown".to_string(),
        last_name: "User".to_string(),
    }
}

pub async fn update_password(request: PasswordToUpdate) -> bool {
    let mut conn = data_access::get_connection();

    let query = "UPDATE users SET password = :password WHERE email = :email";

    let result = conn
        .exec_iter(
            query,
            params! {
            "email" => request.email,
            "password" => request.password
            },
        )
        .expect("Failed to execute register query")
        .affected_rows();

    result == 1
}

//only for tests
pub async fn _get_last_user_id() -> u32 {
    let mut conn = data_access::get_connection();

    let query = "SELECT MAX(user_id) FROM users";

    let result: Option<u32> = conn.query_first(query).expect("Failed to execute query");

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_register_user() {
        let user_to_insert = RegisterRequest {
            email: "test@uv.mx".to_string(),
            name: "test".to_string(),
            last_name: "test".to_string(),
            password: "test".to_string(),
        };
        let result = register_user(user_to_insert).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_get_all_user_emails() {
        let result = get_all_user_emails().await;
        assert!(result.is_empty() == false);
    }

    #[tokio::test]
    async fn test_update_password() {
        let password_to_update = PasswordToUpdate {
            email: "test@uv.mx".to_string(),
            password: "updatedtest".to_string(),
        };

        let result = update_password(password_to_update).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_get_user_name() {
        let result = get_user_name(1).await;
        assert!(result.name.is_empty() == false && result.last_name.is_empty() == false);
    }

    #[tokio::test]
    async fn test_update_user() {
        let user_to_update = UserToUpdate {
            id: _get_last_user_id().await,
            name: "updatedtest".to_string(),
            last_name: "updatedtest".to_string(),
        };

        let result = update_user(user_to_update).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let result = delete_user(_get_last_user_id().await).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_register_user_invalid() {
        let user_to_insert = RegisterRequest {
            email: "test@gmail.com".to_string(),
            name: "test".to_string(),
            last_name: "test".to_string(),
            password: "test".to_string(),
        };
        let result = register_user(user_to_insert).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_get_all_user_emails_invalid() {
        let result = get_all_user_emails().await;
        assert!(result.is_empty() == true);
    }

    #[tokio::test]
    async fn test_update_password_invalid() {
        let password_to_update = PasswordToUpdate {
            email: "test@gmail.com".to_string(),
            password: "updatedtest".to_string(),
        };

        let result = update_password(password_to_update).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_get_user_name_invalid() {
        let result = get_user_name(0).await;
        assert!(result.name.is_empty() == false && result.last_name.is_empty() == false);
    }

    #[tokio::test]
    async fn test_update_user_invalid() {
        let user_to_update = UserToUpdate {
            id: 0,
            name: "updatedtest".to_string(),
            last_name: "updatedtest".to_string(),
        };

        let result = update_user(user_to_update).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete_user_invalid() {
        let result = delete_user(0).await;
        assert!(result);
    }
}
