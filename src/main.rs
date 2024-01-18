use dotenv::dotenv;
use reqwest;
use std::env;
use serde_json;
use std::fs::File;
use std::io::BufWriter; 

mod steam;

use steam::games;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");
    let steamid: u64 = 76561198118055178;

    let http_client: reqwest::Client = reqwest::Client::new();


    let result: Result<String, reqwest::Error> = games::get_owned_games(http_client, steam_api_key, steamid).await;
    let json_result_raw: String = result.unwrap();
    let json_result: serde_json::Value = serde_json::from_str(&json_result_raw).unwrap();
    println!("{}",json_result);

    let file = File::create("temp/owned_games_scruffy.json")?;
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &json_result);

    Ok(())

}