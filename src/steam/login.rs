use dotenv::dotenv;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use steam_connect::{Redirect, Verify};
use std::sync::{Arc, Mutex};
use std::env;

pub struct AppState {
    pub steam_id: Mutex<Option<String>>,
}

pub async fn login() -> Result<HttpResponse> {
    Ok(Redirect::new("http://127.0.0.1:8080/auth/callback")
        .unwrap()
        .redirect())
}

pub async fn callback(req: HttpRequest, data: web::Data<Arc<AppState>>) -> Result<HttpResponse> {
    let verification_result = Verify::verify_request(req.query_string()).await;
    let mut steam_id_lock = data.steam_id.lock().unwrap();
    
    match verification_result {
        Ok(v) => {
            let steam_id = v.claim_id();
            *steam_id_lock = Some(steam_id.to_string());

            Ok(HttpResponse::Ok().body(format!(
                "Hello {}! Your SteamID: {}",
                v.get_summaries("8E059F59D624F5191EA84ECD3F7D610A")
                    .await
                    .unwrap()
                    .personaname,
                steam_id,
            )))
        }
        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("Err: {:?}", e))),
    }
}


pub async fn get_steam_id(data: web::Data<Arc<AppState>>) -> Result<Option<String>, actix_web::Error> {
    let steam_id_lock = data.steam_id.lock().map_err(|_| actix_web::error::ErrorInternalServerError("Lock failed"))?;
    Ok(steam_id_lock.clone())
}

pub async fn use_steam_id(data: web::Data<Arc<AppState>>) -> Result<HttpResponse> {
    match get_steam_id(data).await? {
        Some(steam_id) => Ok(HttpResponse::Ok().body(format!("Using saved SteamID: {}", steam_id))),
        None => Ok(HttpResponse::BadRequest().body("SteamID not found")),
    }
}
