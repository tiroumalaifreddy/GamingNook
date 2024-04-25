use reqwest::{self, header::AUTHORIZATION};
use thirtyfour::prelude::*;
use url::Url;
use std::time::Duration;
use serde_json;
use serde::{Deserialize, Serialize};

pub async fn get_token() -> Result<std::string::String, WebDriverError>{
    // Set up WebDriver session
    let mut caps = DesiredCapabilities::chrome();
    let _ = caps.set_javascript_enabled(true);
    let session = WebDriver::new("http://localhost:9515", caps).await?;
    
    // Step 1: Visit the authentication URL and get the code
    // let auth_url = "https://www.epicgames.com/account/v2/payment/ajaxGetOrderHistory?sortDir=DESC&sortBy=DATE";
    let auth_url = "https://www.epicgames.com/id/login?lang=fr&noHostRedirect=true&redirectUrl=https%3A%2F%2Fstore.epicgames.com%2Ffr%2F&client_id=875a3b57d3a640a6b7f9b4e883463ab4";
    session.goto(auth_url).await?;
    // Let the user log in manually or perform any necessary actions
    // This is where the user might need to enter credentials
    let timeout_duration = Duration::from_secs(90);
    let start_time = tokio::time::Instant::now();

    while session.current_url().await?.as_str() == "https://www.epicgames.com/id/login?lang=fr&noHostRedirect=true&redirectUrl=https%3A%2F%2Fstore.epicgames.com%2Ffr%2F&client_id=875a3b57d3a640a6b7f9b4e883463ab4" {
        if tokio::time::Instant::now() - start_time > timeout_duration {
            return Err(WebDriverError::Timeout("Timeout waiting for URL change".to_string()));
        }

        // Adjust the sleep duration based on your requirements
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    // Wait for the user to finish authentication (you might need to adjust the time)
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Get the current URL, which includes the code in the query parameters
    let current_page = session.source().await?;
    
    // Close the WebDriver session
    session.quit().await?;

    Ok(current_page)
}