mod option;
mod misc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn run_workflow(url: Option<&str>) -> Result<String, String> {
    let driver = option::init_browser().await
        .map_err(|err| format!("{}", err))?;

    option::open_web(&driver, url.unwrap_or("https://wikipedia.org")).await
        .map_err(|err| format!("{}", err))?;

    option::sleep(Some(1000)).await?;
    
    option::handle_banner(&driver).await
        .map_err(|err| format!("{}", err))?;

    option::sleep(Some(5000)).await?;

    driver.quit().await
        .map_err(|err| format!("{}", err))?;

    Ok("good job".to_string()) 
}

#[tauri::command]
async fn get_web_page(url: &str) -> Result<String, String> {
    println!("browse web pages: {}", url);
    return misc::http_get(url).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            run_workflow,
            get_web_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
