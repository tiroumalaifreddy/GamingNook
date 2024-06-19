use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use std::sync::{Arc, Mutex};
use std::thread;
use webbrowser;
mod steam;
mod games;
mod error;
use error::MyError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let shared_state = Arc::new(steam::login::AppState {
        steam_id: Mutex::new(None),
    });

    let server = HttpServer::new(move || {
        let shared_state_clone = shared_state.clone();
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(shared_state_clone))
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(steam::login::login))
                    .route("/callback", web::get().to(steam::login::callback)),
            )
            .route("/check_steam_id", web::get().to(steam::login::check_steam_id))
    })
    .bind("127.0.0.1:8080")?
    .run();

    // Open the web browser to the login page after the server has started
    let url = "http://127.0.0.1:8080/auth/login";
    thread::spawn(move || {
        // Add a small delay to ensure the server has started
        thread::sleep(std::time::Duration::from_secs(1));
        if webbrowser::open(url).is_err() {
            eprintln!("Failed to open browser. Please navigate to {}", url);
        }
    });

    server.await
}
