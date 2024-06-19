use reqwest::{self, header::AUTHORIZATION};
use thirtyfour::prelude::*;
use std::time::Duration;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GogGame {
    pub appid: i64,
    pub title: String
}

type RequestResult = Result<String, String>;

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

    let response = reqwest::Client::new().post(token_url).form(&params).send().await.unwrap();
    let token_json: String = response.text().await.unwrap();

    // Now you have the token_json, and you can extract the access token
    // let access_token = token_json["access_token"].as_str().unwrap_or_default();

    // Close the WebDriver session
    session.quit().await?;
    let parsed_json: serde_json::Value = serde_json::from_str(&token_json).expect("Failed to parse JSON");

    // Extract the access_token
    let access_token: String = parsed_json["access_token"].as_str().unwrap().to_string();

    Ok(access_token)
}

pub async fn get_owned_games_ids(client:&reqwest::Client, gog_token: &String) -> Result<Vec<serde_json::Value>, reqwest::Error>{


    let bearer_token = format!("Bearer {}", gog_token);
    let api_url = "https://embed.gog.com/user/data/games";
    let response: reqwest::Response = client.get(api_url).header(AUTHORIZATION, bearer_token).send().await?;

    let response_json: String = response.text().await?;
    let response_struct: serde_json::Value = serde_json::from_str(&response_json).expect("Failed to parse JSON");
    let owned_games_id: &Vec<serde_json::Value> = response_struct["owned"].as_array().unwrap();


    Ok(owned_games_id.to_vec())

}

pub async fn get_owned_games(client:&reqwest::Client, gog_token: &String, owned_games_id: Vec<serde_json::Value>) -> Result<Vec<GogGame>, reqwest::Error> {
    let bearer_token = format!("Bearer {}", gog_token);
    let mut results: Vec<GogGame> = Vec::new();

    for value in owned_games_id {
        if let Some(number) = value.as_i64() {
            let url_games = format!("https://embed.gog.com/account/gameDetails/{}.json", number);
            let response: reqwest::Response = client.get(url_games).header(AUTHORIZATION, bearer_token.clone()).send().await?;

            let response_json: String = response.text().await?;
            let response_struct: serde_json::Value = serde_json::from_str(&response_json).expect("Failed to parse JSON");
            println!("{}", number);
            let game_gog = GogGame {
                appid: number,
                title: response_struct["title"].as_str().map(|s| s.to_string()).unwrap_or_else(|| format!("Untitled Game"))
            };
            results.push(game_gog)
        } else {
            // Handle the case where the value is not a Number
            println!("Skipping non-numeric value: {:?}", value);
        }
    }

    let new_results: Vec<GogGame> = results
        .into_iter()
        .filter(|game| game.title != "Untitled Game")
        .collect();

    Ok(new_results)
    
}
