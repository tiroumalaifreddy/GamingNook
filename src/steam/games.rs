use dotenv::dotenv;
use reqwest;

pub async fn get_recent_games(steam_api_key: String, steamid: u64) -> Result<String, reqwest::Error>{
    dotenv().ok();
    // Get the Steam API Key as an environment variable
    // let api_url: String = "https://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v1/?key=".to_owned() 
    // + &steam_api_key 
    // + "&steamid=".to_owned();

    let api_url = format!("https://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v1/?key={steam_api_key}&steamid={steamid}");
 
    let response: reqwest::Response = reqwest::get(&api_url).await?;

    // Print the total count of the user's recently played games
    // Check if the request was successful (HTTP status code 200)
    let response_json: String = response.text().await?;
    Ok(response_json)
}