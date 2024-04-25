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
mod epic;

mod games;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let str_page = epic::epicgames::get_token();
    println!("{}",str_page.await?);
    Ok(())

}
