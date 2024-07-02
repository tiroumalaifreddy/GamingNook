use reqwest::{self, header::AUTHORIZATION};
use thirtyfour::prelude::*;

use serde_json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GogGame {
    pub appid: i64,
    pub title: String
}


pub async fn get_token(code_input: String) -> Result<std::string::String, WebDriverError>{

    let code = code_input; 

    // Step 2: Use the obtained code to get the token
    let token_url = "https://auth.gog.com/token";
    let client_id = "46899977096215655"; // Replace with your actual client ID
    let client_secret = "9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9"; // Replace with your actual client secret

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", &code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "https://embed.gog.com/on_login_success?origin=client")
    ];

    let response = reqwest::Client::new().post(token_url).form(&params).send().await.unwrap();
    let token_json: String = response.text().await.unwrap();


    let parsed_json: serde_json::Value = serde_json::from_str(&token_json).expect("Failed to parse JSON");

    let access_token: String = parsed_json["access_token"].as_str().unwrap().to_string();

    Ok(access_token)
}

pub async fn get_userid(client: &reqwest::Client, gog_token: &String) -> Result<String, Box<dyn std::error::Error>> {
    let bearer_token = format!("Bearer {}", gog_token);
    let api_url = "https://embed.gog.com/userData.json";
    let response = client.get(api_url).header(AUTHORIZATION, bearer_token).send().await?;

    if response.status().is_success() {
        let response_json: String = response.text().await?;
        let response_struct: serde_json::Value = serde_json::from_str(&response_json)?;
        
        if let Some(user_id) = response_struct["userId"].as_str() {
            Ok(user_id.to_string())
        } else {
            Err("userId field is missing or not an integer".into())
        }
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
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
            println!("Skipping non-numeric value: {}", value);
        }
    }

    let new_results: Vec<GogGame> = results
        .into_iter()
        .filter(|game| game.title != "Untitled Game")
        .collect();

    Ok(new_results)
    
}
