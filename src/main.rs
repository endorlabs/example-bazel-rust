use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

fn main() {
    let msg = Message {
        content: "Hello, world!".to_string(),
    };

    let json = serde_json::to_string(&msg).unwrap();
    println!("{}", json);
}
