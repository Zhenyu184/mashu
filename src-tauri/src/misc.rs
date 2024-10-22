use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;
use reqwest;

pub async fn http_get(url: &str) -> Result<String, String> {
    let response = timeout(Duration::from_secs(5), reqwest::get(url))
        .await
        .map_err(|_| "Request timed out".to_string())? 
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?; 
        return Ok(body)
    }

    Err(format!("Failed to fetch the page: {}", response.status()))
}

pub fn execute_command(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|err| format!("{}", err))?;

    if output.status.success() {
        let ret = String::from_utf8(output.stdout)
            .map_err(|err| format!("{}", err));
        return ret
    }

    let msg = String::from_utf8_lossy(&output.stderr);
    Err(format!("Command failed with error: {}", msg))
}

pub async fn sleep_milliseconds(milliseconds: u64) {
    tokio::time::sleep(Duration::from_millis(milliseconds)).await;
}
