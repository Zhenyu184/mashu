use reqwest;
use std::time::Duration;
use tokio::time::timeout;

pub async fn http_get(url: &str) -> Result<String, String> {
    let response = timeout(Duration::from_secs(5), reqwest::get(url))
        .await
        .map_err(|_| "Request timed out".to_string())?
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        return Ok(body);
    }

    Err(format!("Failed to fetch the page: {}", response.status()))
}
