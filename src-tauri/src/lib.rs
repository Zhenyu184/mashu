use thirtyfour::prelude::*;
use tokio;

mod misc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name)
}

// 初始化瀏覽器驅動
// 回傳 -> Result<WebDriver, WebDriverError>
async fn init_browser(url: &str) -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto(url).await?;
    driver.maximize_window().await?;    // 最大化視窗
    Ok(driver)
}

// 處理網站橫幅(Cookie)
// 若存在橫幅則點擊拒絕按鈕
async fn handle_banner(driver: &WebDriver) -> WebDriverResult<()> {
    if let Err(_) = driver.find(By::ClassName("banner-content")).await {
        return Ok(());
    }
    
    if let Ok(reject_button) = driver.find(By::Id("onetrust-reject-all-handler")).await {
        reject_button.click().await?;
    }
    
    misc::sleep_milliseconds(100).await;
    Ok(())
}

#[tauri::command]
async fn main_process(url: Option<&str>) -> Result<String, String> {
    println!("main process log 1");

    let url = url.unwrap_or("https://wikipedia.org");
    
    println!("main process log 2");
    let driver = init_browser(url).await.map_err(|err| {
        eprintln!("{}", err);
        format!("{}", err)
    })?;
    
    println!("main process log 3");
    handle_banner(&driver).await
        .map_err(|err| format!("{}", err))?;

    misc::sleep_milliseconds(2000).await;

    println!("main process log 4");
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
