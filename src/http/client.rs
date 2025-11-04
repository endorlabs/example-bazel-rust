use anyhow::Result;
use serde_json::json;
use tokio::time::{sleep, Duration};
use super::response::ApiResponse;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_json(&self, url: &str) -> Result<ApiResponse> {
        let response = self.client.get(url).send().await?;
        let status = if response.status().is_success() {
            "success"
        } else {
            "error"
        };
        let data = response.json().await?;
        
        Ok(ApiResponse {
            status: status.to_string(),
            data,
        })
    }

    pub async fn get_json_with_retry(&self, url: &str, retries: u32) -> Result<ApiResponse> {
        for attempt in 1..=retries {
            match self.get_json(url).await {
                Ok(response) => return Ok(response),
                Err(e) if attempt == retries => return Err(e),
                Err(_) => {
                    sleep(Duration::from_millis(100 * attempt as u64)).await;
                }
            }
        }
        unreachable!()
    }

    pub fn create_test_response() -> ApiResponse {
        ApiResponse {
            status: "success".to_string(),
            data: json!({"message": "test"}),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

