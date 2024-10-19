use std::time::Duration;
use tokio::time::timeout;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_web_page(url: &str) -> Result<String, String> {
    println!("browse web pages: {}", url);

    // timeout 5s
    let response = timeout(Duration::from_secs(5), reqwest::get(url))
        .await
        .map_err(|_| "Request timed out".to_string())? 
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("Failed to fetch the page: {}", response.status()))
    }
}

#[tauri::command]
fn input_string(body: &str, search: &str, string: &str,) -> Result<String, String> {
    println!("input string {}", string);
    if true {
        Ok(format!("logged_in {}", string))
    } else {
        Err("invalid credentials".to_string())
    }
}

#[tauri::command]
fn press_button_component(body: &str, search: &str) -> Result<String, String> {
    println!("press {} button component", search);
    if true {
        Ok(format!("logged_in {}", search))
    } else {
        Err("invalid credentials".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_web_page, input_string, press_button_component])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
