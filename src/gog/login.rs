use actix_session::Session;
use actix_web::web::Redirect;
use actix_web::{web, HttpResponse, Responder, Result};
use crate::authentication::auth;

use rusqlite::{params, Connection};


use crate::MyError;
use crate::games;
use crate::gog::goggames;
use serde::Deserialize;

pub async fn login() -> impl Responder{
    Redirect::to("https://auth.gog.com/auth?client_id=46899977096215655&redirect_uri=https%3A%2F%2Fembed.gog.com%2Fon_login_success%3Forigin%3Dclient&response_type=code&layout=client2")
        .permanent()
}

#[derive(Deserialize)]
pub struct CodeQuery {
    code_input: String,
}

pub async fn handle_code_temp(session: Session ,query: web::Query<CodeQuery>) -> Result<HttpResponse, MyError> {
    let code_input = &query.code_input;
    print!("{}", code_input);
    let toktok = goggames::get_token(code_input.to_string()).await.unwrap();
    let http_client: reqwest::Client = reqwest::Client::new();
    let games_id = goggames::get_owned_games_ids(&http_client, &toktok);
    let gogid : String = goggames::get_userid(&http_client, &toktok).await.unwrap();
    let owned_games_id: Vec<serde_json::Value> = games_id.await.unwrap();
    let result = goggames::get_owned_games(&http_client, &toktok, owned_games_id).await?;
    let user_id = auth::validate_session(&session).unwrap();
    let conn = Connection::open("temp/test.db3")?;
    conn.execute_batch(
        r"CREATE TABLE IF NOT EXISTS game (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            userid INTEGER NOT NULL,
            appid INTEGER NOT NULL,
            name TEXT NOT NULL,
            playtime INTEGER,
            platform TEXT NOT NULL,
            FOREIGN KEY(userid) REFERENCES users(id)
        );"
    )?;

    let mut stmt = conn.prepare("UPDATE users SET gogid = ? WHERE id = ?").expect("Failed to prepare statement");
    stmt.execute(params![gogid, user_id])?;
    

    let games_format = games::Games::from_gog_games(result, user_id.to_string());

    for game in games_format.games {
        conn.execute(
            "INSERT INTO game (userid, appid, name, playtime, platform) VALUES (?, ?, ?, ?, ?)",
            params![game.userid, game.appid, game.name, game.playtime, game.platform],
        )?;
    }


    println!("Received code: {}", toktok);
    Ok(HttpResponse::Ok().body(format!("Received code: {}", toktok)))
}
