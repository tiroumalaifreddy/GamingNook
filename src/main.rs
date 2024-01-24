use dotenv::dotenv;
use reqwest;
use steam::steamgames::SteamGame;
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
    let toktok = gog::test::get_token();
    println!("token is {}", toktok.await?);
    Ok(())

}
