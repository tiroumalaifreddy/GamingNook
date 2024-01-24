use dotenv::dotenv;
use reqwest;
use steam::steamgames::SteamGame;
use std::env;
use serde_json;
use std::fs::File;
use std::io::BufWriter;

use duckdb::{params, Connection, Result};

mod steam;

mod games;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();

    let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");
    let steamid: u64 = 76561198118055178;

    let http_client: reqwest::Client = reqwest::Client::new();

    let result: Result<Vec<SteamGame>, reqwest::Error> = steam::steamgames::get_owned_games(http_client, steam_api_key, steamid).await;

    let conn: Connection = Connection::open("temp/test.db")?;

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE IF NOT EXISTS game (
                  appid              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  playtime INTEGER,
                  platform TEXT NOT NULL
                  );
        ")?;
    
    let unwraped_result: Vec<SteamGame> = result.unwrap();

    let games_format: games::Games = games::Games::from_steam_games(unwraped_result);
                  



    for element  in games_format.games {
        conn.execute(
            "INSERT INTO game (appid, name, playtime, platform) VALUES (?, ?, ?, ?)",
            params![element.appid, element.name, element.playtime, element.platform],
        )?;
    }
    let _ = conn.close();

    Ok(())

}
