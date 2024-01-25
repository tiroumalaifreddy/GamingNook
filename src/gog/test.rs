use reqwest::{self, header::AUTHORIZATION};
use thirtyfour::prelude::*;
use url::Url;
use std::time::Duration;
use serde_json;


pub async fn get_token() -> Result<std::string::String, WebDriverError>{
    // Set up WebDriver session
    let caps = DesiredCapabilities::firefox();
    let session = WebDriver::new("http://localhost:4444", caps).await?;
    
    // Step 1: Visit the authentication URL and get the code
    let auth_url = "https://auth.gog.com/auth?client_id=46899977096215655&redirect_uri=https%3A%2F%2Fembed.gog.com%2Fon_login_success%3Forigin%3Dclient&response_type=code&layout=client2";
    session.goto(auth_url).await?;
    // Let the user log in manually or perform any necessary actions
    // This is where the user might need to enter credentials
    let timeout_duration = Duration::from_secs(90);
    let start_time = tokio::time::Instant::now();

    while session.current_url().await?.as_str() == "https://login.gog.com/auth?client_id=46899977096215655&layout=client2&redirect_uri=https%3A%2F%2Fembed.gog.com%2Fon_login_success%3Forigin%3Dclient&response_type=code" {
        if tokio::time::Instant::now() - start_time > timeout_duration {
            return Err(WebDriverError::Timeout("Timeout waiting for URL change".to_string()));
        }

        // Adjust the sleep duration based on your requirements
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    // Wait for the user to finish authentication (you might need to adjust the time)
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Get the current URL, which includes the code in the query parameters
    let current_url = session.current_url().await?;
    
    // Extract the code from the URLcargo run --example tokio_async

    let url_str: String = current_url.to_string();
    let url = Url::parse(&url_str).expect("Failed to parse URL");
    let url_parts: Vec<&str> = url_str.split('?').collect();
    let query_params: Vec<&str> = url_parts[1].split('&').collect();
    let mut code = "";
    for param in query_params {
        let key_value: Vec<&str> = param.split('=').collect();
        if key_value[0] == "code" {
            code = key_value[1];
            break;
        }
    }
    // Step 2: Use the obtained code to get the token
    let token_url = "https://auth.gog.com/token";
    let client_id = "46899977096215655"; // Replace with your actual client ID
    let client_secret = "9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9"; // Replace with your actual client secret

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "https://embed.gog.com/on_login_success?origin=client")
    ];

    let response = reqwest::Client::new().post(token_url).form(&params).send().await?;
    let token_json: String = response.text().await?;

    // Now you have the token_json, and you can extract the access token
    // let access_token = token_json["access_token"].as_str().unwrap_or_default();

    // Close the WebDriver session
    session.quit().await?;
    let parsed_json: serde_json::Value = serde_json::from_str(&token_json).expect("Failed to parse JSON");

    // Extract the access_token
    let access_token: String = parsed_json["access_token"].as_str().unwrap().to_string();

    Ok(access_token)
}

pub async fn get_owned_games(client:reqwest::Client, gog_token: String) -> Result<String, reqwest::Error>{


    let bearer_token = format!("Bearer {}", gog_token);
    let api_url = "https://embed.gog.com/user/data/games";
    let response: reqwest::Response = client.get(api_url).header(AUTHORIZATION, bearer_token).send().await?;

    let response_json: String = response.text().await?;
    let response_struct: serde_json::Value = serde_json::from_str(&response_json).expect("Failed to parse JSON");
    let owned_games_id = response_struct["owned"].as_array().unwrap();
    println!("{:?}", owned_games_id);

    Ok(response_json)

}