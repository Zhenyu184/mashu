use thirtyfour::prelude::*;

use crate::misc;

pub async fn sleep(milliseconds: Option<u64>) -> Result<(), String> {
    let millis = milliseconds.unwrap_or(0);
    misc::sleep_milliseconds(millis).await;
    Ok(())
}

// 初始化瀏覽器驅動
pub async fn init_browser() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    Ok(driver)
}

// 打开指定的 URL
pub async fn open_web(driver: &WebDriver, url: &str) -> WebDriverResult<()> {
    driver.goto(url).await?;
    driver.maximize_window().await?;
    Ok(())
}

// 處理網站橫幅 e.g. Cookie
// 若存在橫幅則點擊拒絕按鈕
pub async fn handle_banner(driver: &WebDriver) -> WebDriverResult<()> {
    if let Err(_) = driver.find(By::ClassName("banner-content")).await {
        return Ok(());
    }
    
    if let Ok(reject_button) = driver.find(By::Id("onetrust-reject-all-handler")).await {
        reject_button.click().await?;
    }
    
    Ok(())
}

pub async fn input_text(driver: &WebDriver, element_id: &str, text: &str) -> WebDriverResult<()> {
    let input_element = driver.find(By::Id(element_id)).await?;
    input_element.send_keys(text).await?;
    Ok(())
}

pub async fn click_button(driver: &WebDriver, button_id: &str) -> WebDriverResult<()> {
    let button_element = driver.find(By::Id(button_id)).await?;
    button_element.click().await?;
    Ok(())
}