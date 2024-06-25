use egs_api::EpicGames;
use crate::MyError;
use crate::games;
use serde::Deserialize;
use rusqlite::{params, Connection};
use actix_web::web::Redirect;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};

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
    let games_epic_raw = egs.library_items(false).await;
    let games_epic = games_epic_raw.unwrap().records;
    
    Ok(HttpResponse::Ok().body(format!("Received code: {:?}", games_epic)))
}
