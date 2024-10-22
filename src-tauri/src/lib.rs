mod option;
mod misc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn main_process(url: Option<&str>) -> Result<String, String> {
    let url = url.unwrap_or("https://wikipedia.org");
    let driver = option::init_browser(url).await.map_err(|err| {
        eprintln!("{}", err);
        format!("{}", err)
    })?;

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

#[tauri::command]
fn input_string(body: &str, search: &str, string: &str,) -> Result<String, String> {
    println!("input string {}", string);
    if true {
        return Ok(format!("logged_in {}", string))
    } 

    Err("invalid credentials".to_string())
}

#[tauri::command]
fn press_button_component(body: &str, search: &str) -> Result<String, String> {
    println!("press {} button component", search);
    if true {
        return Ok(format!("logged_in {}", search))
    }
    
    return Err("invalid credentials".to_string())
}

#[tauri::command]
fn choose_drop_down_component(body: &str, search: &str) -> Result<String, String> {
    println!("choose {} drop down component", search);
    if true {
        return Ok(format!("logged_in {}", search))
    }
    
    return Err("invalid credentials".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            main_process,
            get_web_page,
            input_string,
            press_button_component,
            choose_drop_down_component
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
