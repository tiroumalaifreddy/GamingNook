use actix_web::web::Redirect;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use dotenv::dotenv;
use reqwest::Client;
use rusqlite::{params, Connection};
use std::env;
use std::sync::{Arc, Mutex};
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
    code: String,
}

pub async fn handle_code_temp(query: web::Query<CodeQuery>) -> Result<HttpResponse> {
    let code = &query.code;
    // Here you can handle the "code" parameter as needed
    // For example, you might want to validate the code, exchange it for a token, etc.
    println!("Received code: {}", code);
    Ok(HttpResponse::Ok().body(format!("Received code: {}", code)))
}
