use dotenvy::dotenv;
use serde_json::json;
use reqwest::Client;
use std::env;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref VERIFICATION_CODES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn generate_verification_code() -> String {
    let code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    code
}

pub async fn send_verification_email(email: String, code: String) {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found");
    let from_email = "studyvaultuv@gmail.com";

    let body = json!({
        "personalizations": [{
            "to": [{ "email": email }],
            "subject": "Verification code for Study Vault"
        }],
        "from": { "email": from_email },
        "content": [{
            "type": "text/plain",
            "value": format!("Your verification code is: {}", code)
        }]
    });

    let client = Client::new();
    let response = client.post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Email sent successfully!");
            } else {
                println!("Failed to send email: {}", res.status());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}