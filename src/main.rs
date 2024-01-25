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
    let toktok = gog::test::get_token().await?;
    
    let http_client: reqwest::Client = reqwest::Client::new();
    let games_id = gog::test::get_owned_games(http_client, toktok);
    println!("{:?}", games_id.await?);
    Ok(())

}
