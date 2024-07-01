use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use dotenv::dotenv;
use env_logger::Env;
use std::sync::{Arc, Mutex};
use std::thread;
use webbrowser;
mod steam;
mod gog;
mod games;
mod authentication;
mod epic;
mod error;
use error::MyError;

async fn login_form() -> HttpResponse {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Login</title>
        </head>
        <body>
            <h1>Login</h1>
            <form id="loginForm">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username"><br><br>
                <label for="password">Password:</label>
                <input type="password" id="password" name="password"><br><br>
                <input type="submit" value="Login">
            </form>
            <script>
                document.getElementById('loginForm').addEventListener('submit', function(event) {
                    event.preventDefault();
                    const username = document.getElementById('username').value;
                    const password = document.getElementById('password').value;
                    
                    fetch('/users/login', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({ username, password })
                    })
                    .then(response => response.json())
                    .then(data => {
                        console.log('Success:', data);
                    })
                    .catch((error) => {
                        console.error('Error:', error);
                    });
                });
            </script>
        </body>
        </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

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
            .wrap(
                SessionMiddleware::new(
                    redis_store.clone(),
                    secret_key.clone(),
                )
            )
            .default_service(web::to(|| HttpResponse::Ok()))
            .app_data(web::Data::new(shared_state_clone.clone()))
            .route("/index", web::get().to(authentication::auth::index))
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
            .route("/login_form", web::get().to(login_form))  // New route for login form
    })
    .bind("127.0.0.1:8080")?
    .run();

    let url = "http://127.0.0.1:8080/auth/epic/login";
    // thread::spawn(move || {
    //     thread::sleep(std::time::Duration::from_secs(1));
    //     if webbrowser::open(url).is_err() {
    //         eprintln!("Failed to open browser. Please navigate to {}", url);
    //     }
    // });

    server.await
}
