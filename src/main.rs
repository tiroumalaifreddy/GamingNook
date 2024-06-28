use actix_web::{middleware, web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use dotenv::dotenv;
use env_logger::Env;
use std::sync::{Arc, Mutex};
use std::thread;
use actix_web::cookie::Key;
use webbrowser;
mod steam;
mod gog;
mod games;
mod authentication;
mod epic;
mod error;
use error::MyError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let shared_state = Arc::new(steam::login::AppState {
        steam_id: Mutex::new(None),
    });

    let secret_key = Key::generate();

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();

    let server = HttpServer::new(move || {
        let shared_state_clone = shared_state.clone();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::new(redis_store.clone(), secret_key.clone()))
            .app_data(web::Data::new(shared_state_clone))
            .service(
                web::scope("/auth/steam")
                    .route("/login", web::get().to(steam::login::login))
                    .route("/callback", web::get().to(steam::login::callback)),
            )
            .service(
                web::scope("/auth/gog")
                    .route("/login", web::get().to(gog::login::login))
                    .route("/code_temp", web::get().to(gog::login::handle_code_temp)),
            )
            .service(
                web::scope("/auth/epic")
                    .route("/login", web::get().to(epic::login::login))
                    .route("/code_temp", web::get().to(epic::login::handle_code_temp)),
            )
            .route("/check_steam_id", web::get().to(steam::login::check_steam_id))
            .service(
                web::scope("/users")
                    .route("/register", web::post().to(authentication::auth::register))
                    .route("/login", web::post().to(authentication::auth::login)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();

    let url = "http://127.0.0.1:8080/auth/epic/login";
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(1));
        if webbrowser::open(url).is_err() {
            eprintln!("Failed to open browser. Please navigate to {}", url);
        }
    });

    server.await
}
