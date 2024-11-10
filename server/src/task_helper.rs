use thirtyfour::{WebDriver, By, WebElement, error::{WebDriverError, WebDriverErrorInfo}};
// use tokio::runtime::Runtime;

pub async fn find_component(driver: &WebDriver, selector: &str) -> Result<WebElement, WebDriverError> {
    match (
        driver.find(By::Id(selector)).await,
        driver.find(By::Name(selector)).await,
        driver.find(By::Css(selector)).await,
    ) {
        (Ok(elem), _, _) | (_, Ok(elem), _) | (_, _, Ok(elem)) => Ok(elem),
        (Err(_), Err(_), Err(_)) => Err(WebDriverError::NoSuchElement(
            WebDriverErrorInfo::new(format!("Element not found with selector: {}", selector))
        ))
    }
}

// pub fn quit_driver(driver: WebDriver) {
//     let rt = Runtime::new().expect("Failed to create runtime");
//     let _ = rt.block_on(async {
//         driver.quit().await
//     });
// }

// pub async fn wait_for_element(driver: &WebDriver, selector: &str, timeout_secs: u64) -> Result<WebElement, WebDriverError> {
//     let elem = driver.query(By::Css(selector))
//         .wait(std::time::Duration::from_secs(timeout_secs))
//         .first()
//         .await?;
//     Ok(elem)
// } 