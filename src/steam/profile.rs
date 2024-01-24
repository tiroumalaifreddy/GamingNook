use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Player {
    steamid: String,
    profilestate: u32,
    personaname: String,
    profileurl: String,
    avatarmedium: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    response: Vec<Player>
}



pub async fn get_player_summaries(client:reqwest::Client,steam_api_key: String, steamid: u64) -> Result<Option<Player>, reqwest::Error>{

    let api_url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={steam_api_key}&steamids={steamid}");
 
    let response: reqwest::Response = client.get(&api_url).send().await?;

    let response_json: String = response.text().await?;
    let response_raw: Response = serde_json::from_str(&response_json).unwrap();
    if let Some(player) = response_raw.response.first() {
        Ok(Some(player.clone()))
    } else {
        Ok(None)
    }
}