use actix_web::{web,HttpRequest, HttpResponse, Result};
use dotenv::dotenv;
use reqwest::Client;
use rusqlite::{params, Connection};
use std::env;
use std::sync::{Arc, Mutex};
use steam_connect::{Redirect, Verify};
use crate::MyError;
use crate::games;
use crate::steam::steamgames;

pub struct AppState {
    pub steam_id: Mutex<Option<String>>,
}

pub async fn login() -> Result<HttpResponse> {
    Ok(Redirect::new("http://127.0.0.1:8080/auth/steam/callback")
        .unwrap()
        .redirect())
}

pub async fn callback(req: HttpRequest, data: web::Data<Arc<AppState>>) -> Result<HttpResponse, MyError> {
    let verification_result = Verify::verify_request(req.query_string()).await;
    let mut steam_id_lock = data.steam_id.lock().unwrap();

    match verification_result {
        Ok(v) => {
            let steam_id = v.claim_id();
            *steam_id_lock = Some(steam_id.to_string());

            dotenv().ok();
            let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");

            let http_client = Client::new();
            let result = steamgames::get_owned_games(http_client, steam_api_key.clone(), steam_id.clone()).await?;

            let conn = Connection::open("temp/test.db3")?;
            conn.execute_batch(
                r"CREATE TABLE IF NOT EXISTS users (
                    userid INTEGER PRIMARY KEY AUTOINCREMENT,
                    steamid TEXT UNIQUE,
                    gogid TEXT UNIQUE

                );
                CREATE TABLE IF NOT EXISTS game (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    userid INTEGER NOT NULL,
                    appid INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    playtime INTEGER,
                    platform TEXT NOT NULL,
                    FOREIGN KEY(userid) REFERENCES users(userid)
                );"
            )?;

            let mut stmt = conn.prepare("INSERT OR IGNORE INTO users (steamid) VALUES (?)")?;
            stmt.execute(params![steam_id])?;
            
            let userid: i64 = conn.query_row(
                "SELECT userid FROM users WHERE steamid = ?",
                params![steam_id],
                |row| row.get(0)
            )?;

            let games_format = games::Games::from_steam_games(result, userid);

            for game in games_format.games {
                conn.execute(
                    "INSERT INTO game (userid, appid, name, playtime, platform) VALUES (?, ?, ?, ?, ?)",
                    params![game.userid, game.appid, game.name, game.playtime, game.platform],
                )?;
            }

            Ok(HttpResponse::Ok().body(format!(
                "Hello {}! Your SteamID: {}",
                v.get_summaries(&steam_api_key)
                    .await
                    .unwrap()
                    .personaname,
                steam_id,
            )))
        }
        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("Err: {:?}", e))),
    }
}

pub async fn check_steam_id(data: web::Data<Arc<AppState>>) -> Result<HttpResponse> {
    let steam_id_lock = data.steam_id.lock().unwrap();
    if let Some(ref steam_id) = *steam_id_lock {
        Ok(HttpResponse::Ok().body(format!("Steam ID: {}", steam_id)))
    } else {
        Ok(HttpResponse::BadRequest().body("Steam ID not found"))
    }
}

