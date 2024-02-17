use dotenv::dotenv;
use reqwest;
use steam::steamgames::SteamGame;
use thirtyfour::session::http;
use std::env;
use serde_json;
use std::fs::File;
use std::io::BufWriter;

use duckdb::{params, Connection, Result};

mod steam;
mod gog;

mod games;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let toktok = gog::goggames::get_token().await?;
    
    dotenv().ok();
    let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");
    let steamid: u64 = 76561198118055178;

    let http_client: reqwest::Client = reqwest::Client::new();
    
    let games_id = gog::goggames::get_owned_games_ids(&http_client, &toktok);
    let owned_games_id: Vec<serde_json::Value> = games_id.await?;
    let games_gog = gog::goggames::get_owned_games(&http_client, &toktok, owned_games_id).await;
    
    let games_steam: Result<Vec<SteamGame>, reqwest::Error> = steam::steamgames::get_owned_games(http_client, steam_api_key, steamid).await;
    
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
    
    let unwraped_result_gog: Vec<gog::goggames::GogGame> = games_gog.unwrap();
    let gog_games_format: games::Games = games::Games::from_gog_games(unwraped_result_gog);

    let unwraped_result_steam: Vec<SteamGame> = games_steam.unwrap();
    let steam_games_format: games::Games = games::Games::from_steam_games(unwraped_result_steam);
                  



    for element  in gog_games_format.games {
        conn.execute(
            "INSERT INTO game (appid, name, playtime, platform) VALUES (?, ?, ?, ?)",
            params![element.appid, element.name, element.playtime, element.platform],
        )?;
    }
    for element  in steam_games_format.games {
        conn.execute(
            "INSERT INTO game (appid, name, playtime, platform) VALUES (?, ?, ?, ?)",
            params![element.appid, element.name, element.playtime, element.platform],
        )?;
    }
    let _ = conn.close();
    Ok(())

}
