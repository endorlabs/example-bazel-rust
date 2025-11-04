mod utils;
mod http;
mod db;

use serde::{Deserialize, Serialize};
use serde_json;

use utils::{validate_email, generate_id, sanitize_input};
use http::HttpClient;
use db::Database;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

fn main() {
    // Use utils module
    let email = "test@example.com";
    println!("Email '{}' is valid: {}", email, validate_email(email));
    println!("Generated ID: {}", generate_id());
    println!("Sanitized input: '{}'", sanitize_input("  HELLO WORLD  "));

    // Use http module
    let client = HttpClient::new();
    let test_response = HttpClient::create_test_response();
    println!("Test HTTP response: {:?}", test_response);

    // Use db module
    let mut db = Database::new();
    db.insert("First record".to_string());
    db.insert("Second record".to_string());
    println!("Database has {} records", db.len());

    // Original message
    let msg = Message {
        content: "Hello, world!".to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    println!("{}", json);
}
