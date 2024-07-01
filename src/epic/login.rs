use egs_api::EpicGames;
use crate::MyError;
use crate::games;
use serde::Deserialize;
use rusqlite::{params, Connection};
use actix_web::web::Redirect;
use actix_web::{web, HttpResponse, Responder, Result};

pub async fn login() -> impl Responder{
    Redirect::to("https://www.epicgames.com/id/login?redirectUrl=https%3A%2F%2Fwww.epicgames.com%2Fid%2Fapi%2Fredirect%3FclientId%3D34a02cf8f4414e29b15921876da36f9a%26responseType%3Dcode")
        .permanent()
}

#[derive(Deserialize)]
pub struct CodeQuery {
    code_input: String,
}


pub async fn handle_code_temp(query: web::Query<CodeQuery>) -> Result<HttpResponse, MyError> {
    let code_input = &query.code_input;
    let sid = code_input.trim().to_string();
    let sid_transform = sid.replace(|c: char| c == '"', "");
    println!("Using Auth Code: {}", sid_transform);

    let mut egs = EpicGames::new();
    if egs.auth_code(None, Some(sid)).await {
        println!("Logged in");
    }
    egs.login().await;
    let account_id = egs.user_details().account_id.unwrap_or_default();
    let games_epic_raw = egs.library_items(true).await;
    let games_epic = games_epic_raw.unwrap().records;
     
    // let mut games_format = games::Games::from_epic_games(games_epic, account_id);
    // games_format.remove_duplicates();

    let conn = Connection::open("temp/test.db3")?;
    conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS users (
            userid INTEGER PRIMARY KEY AUTOINCREMENT,
            steamid TEXT UNIQUE,
            gogid TEXT UNIQUE,
            epicid TEXT UNIQUE
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

    let mut stmt = conn.prepare("INSERT OR IGNORE INTO users (epicid) VALUES (?)")?;
    stmt.execute(params![account_id])?;
    
    let userid: i64 = conn.query_row(
        "SELECT userid FROM users WHERE epicid = ?",
        params![account_id],
        |row| row.get(0)
    )?;

    let mut games_format = games::Games::from_epic_games(games_epic.clone(), userid.to_string());
    games_format.remove_duplicates();

    for game in games_format.games {
        conn.execute(
            "INSERT INTO game (userid, appid, name, playtime, platform) VALUES (?, ?, ?, ?, ?)",
            params![game.userid, game.appid, game.name, game.playtime, game.platform],
        )?;
    }

    Ok(HttpResponse::Ok().body(format!("Received code: {:?}", &games_epic)))
}
