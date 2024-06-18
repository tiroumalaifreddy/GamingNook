use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use env_logger::Env;
use reqwest::Client;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::env;
use std::sync::{Arc, Mutex};
use steam_connect::{Redirect, Verify};
use steam::steamgames::SteamGame;
mod steam;
mod games;
mod error;
use error::MyError;

struct AppState {
    steam_id: Mutex<Option<String>>,
}

async fn login() -> Result<HttpResponse> {
    Ok(Redirect::new("http://127.0.0.1:8080/auth/callback")
        .unwrap()
        .redirect())
}

async fn callback(req: HttpRequest, data: web::Data<Arc<AppState>>) -> Result<HttpResponse, MyError> {
    let verification_result = Verify::verify_request(req.query_string()).await;
    let mut steam_id_lock = data.steam_id.lock().unwrap();

    match verification_result {
        Ok(v) => {
            let steam_id = v.claim_id();
            *steam_id_lock = Some(steam_id.to_string());

            dotenv().ok();
            let steam_api_key: String = env::var("STEAM_API_KEY").expect("Missing an API key");

            // Example Steam API request after login
            let http_client = Client::new();
            let result = steam::steamgames::get_owned_games(http_client, steam_api_key.clone(), steam_id.clone()).await?;

            let conn = Connection::open("temp/test.db3")?;
            conn.execute_batch(
                r"CREATE TABLE IF NOT EXISTS game (
                    appid INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    playtime INTEGER,
                    platform TEXT NOT NULL
                );")?;

            let games_format = games::Games::from_steam_games(result);

            for game in games_format.games {
                conn.execute(
                    "INSERT INTO game (appid, name, playtime, platform) VALUES (?, ?, ?, ?)",
                    params![game.appid, game.name, game.playtime, game.platform],
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

async fn check_steam_id(data: web::Data<Arc<AppState>>) -> Result<HttpResponse> {
    let steam_id_lock = data.steam_id.lock().unwrap();
    if let Some(ref steam_id) = *steam_id_lock {
        Ok(HttpResponse::Ok().body(format!("Steam ID: {}", steam_id)))
    } else {
        Ok(HttpResponse::BadRequest().body("Steam ID not found"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let shared_state = Arc::new(AppState {
        steam_id: Mutex::new(None),
    });

    HttpServer::new(move || {
        let shared_state_clone = shared_state.clone();
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(shared_state_clone))
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(login))
                    .route("/callback", web::get().to(callback)),
            )
            .route("/check_steam_id", web::get().to(check_steam_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

