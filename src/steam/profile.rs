use reqwest;

pub async fn get_player_summaries(client:reqwest::Client,steam_api_key: String, steamid: u64) -> Result<String, reqwest::Error>{

    let api_url = format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={steam_api_key}&steamids={steamid}");
 
    let response: reqwest::Response = client.get(&api_url).send().await?;

    let response_json: String = response.text().await?;
    Ok(response_json)
}