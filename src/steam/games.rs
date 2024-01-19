use reqwest;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamGame {
    pub appid: u64,
    pub img_icon_url: String,
    pub name: String,
    pub playtime_disconnected: u64,
    pub playtime_forever: u64,
    pub rtime_last_played: u64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamGameResponse {
    game_count: u64,
    games: Vec<SteamGame>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamJsonResponse {
    response: SteamGameResponse
}

pub async fn get_recent_games(client:reqwest::Client, steam_api_key: String, steamid: u64) -> Result<String, reqwest::Error>{

    let api_url: String = format!("https://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v1/?key={steam_api_key}&steamid={steamid}");
 
    let response: reqwest::Response = client.get(&api_url).send().await?;

    let response_json: String = response.text().await?;

    Ok(response_json)
}

pub async fn get_owned_games(client:reqwest::Client, steam_api_key: String, steamid: u64) -> Result<Vec<SteamGame>, reqwest::Error>{

    let api_url: String = format!("https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?key={steam_api_key}&steamid={steamid}&include_appinfo=true&include_played_free_games=false&appids_filter&langage=english&include_extended_appinfo=false&include_free_sub=false");
 
    let response: reqwest::Response = client.get(&api_url).send().await?;

    let response_json: String = response.text().await?;

    let games_raw: SteamJsonResponse = serde_json::from_str(&response_json).unwrap();
    let games: Vec<SteamGame> = games_raw.response.games;

    Ok(games)
}

pub async fn get_player_achievements(client:reqwest::Client, steam_api_key: String, steamid: u64, appid:u32) -> Result<String, reqwest::Error>{

    let api_url: String = format!("https://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v1/?key={steam_api_key}&steamid={steamid}&appid={appid}");
 
    let response: reqwest::Response = client.get(&api_url).send().await?;

    let response_json: String = response.text().await?;
    Ok(response_json)
}