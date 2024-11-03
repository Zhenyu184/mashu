mod application;
mod misc;
mod task;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[tauri::command]
async fn get_web_page(url: &str) -> Result<String, String> {
    println!("browse web pages: {}", url);
    return misc::http_get(url).await;
}

#[tauri::command]
async fn run_workflow(script: &str) -> Result<String, String> {
    match application::app(script) {
        Ok(_) => Ok("run success".to_string()),
        Err(e) => Err(format!("error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_web_page, run_workflow])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
