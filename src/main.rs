use dotenv::dotenv;
use reqwest;
use std::env;
use serde_json;
use std::fs::File;
use std::io::BufWriter;

use duckdb::{params, Connection, Result};

mod steam;

use steam::games;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");
    let steamid: u64 = 76561198118055178;

    let http_client: reqwest::Client = reqwest::Client::new();

    let result: Result<Vec<games::SteamGame>, reqwest::Error> = games::get_owned_games(http_client, steam_api_key, steamid).await;

    let conn: Connection = Connection::open("temp/test.db")?;

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE steamgame (
                  appid              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  img_icon_url            TEXT NOT NULL,
                  name            TEXT NOT NULL,
                  playtime_disconnected INTEGER,
                  playtime_forever INTEGER,
                  rtime_last_played INTEGER
                  );
        ")?;
    
    let unwraped_result: Vec<games::SteamGame> = result.unwrap();

    for element  in &unwraped_result {
        conn.execute(
            "INSERT INTO steamgame (appid, img_icon_url, name, playtime_disconnected, playtime_forever, rtime_last_played) VALUES (?, ?, ?, ?, ?, ?)",
            params![element.appid, element.img_icon_url, element.name, element.playtime_disconnected, element.playtime_forever, element.rtime_last_played],
        )?;
    }
    let _ = conn.close();

    let file = File::create("temp/owned_games_scruffy.json")?;
    let mut writer = BufWriter::new(file);
    let _ = serde_json::to_writer(&mut writer, &unwraped_result);

    Ok(())

}
