use dotenv::dotenv;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    dotenv().ok();

    let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");
    // Get the Steam API Key as an environment variable
    let api_url: String = "https://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v1/?key=".to_owned() + &steam_api_key + "&steamid=76561198118055178";
 
    let response: reqwest::Response = reqwest::get(&api_url).await?;

    // Print the total count of the user's recently played games
    // Check if the request was successful (HTTP status code 200)
    let response_json: String = response.text().await?;
    println!("{}", response_json);
    Ok(())
}