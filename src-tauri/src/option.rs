use thirtyfour::prelude::*;

use crate::misc;

// 初始化瀏覽器驅動
// 回傳 -> Result<WebDriver, WebDriverError>
pub async fn init_browser(url: &str) -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto(url).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

// 處理網站橫幅(Cookie)
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

pub async fn sleep(milliseconds: Option<u64>) -> Result<(), String> {
    let millis = milliseconds.unwrap_or(0);
    misc::sleep_milliseconds(millis).await;
    Ok(())
}