use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use actix_session::{storage::RedisSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,Result
};

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: i64,
    username: String,
    password: String,
    steamid: String,
    gogid: String,
    epicid: String
}

impl User {
    fn authenticate(credentials: Credentials) -> Result<Self, HttpResponse> {
        let conn = Connection::open("temp/test.db3").unwrap();
        let mut stmt = conn.prepare("SELECT id, password, steamid, gogid, epicid FROM users WHERE username = ?1").unwrap();
        let mut user_rows = stmt.query(params![credentials.username]).unwrap();

        if let Some(row) = user_rows.next().unwrap() {
            let id: i64 = row.get(0).unwrap();
            let db_password: String = row.get(1).unwrap();
            let steam_id: String  = row.get(2).unwrap_or(String::new());
            let gog_id: String = row.get(3).unwrap_or(String::new());
            let epic_id: String = row.get(4).unwrap_or(String::new());

            if verify(&credentials.password, &db_password).unwrap() {
                return Ok(User {
                    id,
                    username: credentials.username,
                    password: db_password,
                    steamid: steam_id,
                    gogid: gog_id,
                    epicid: epic_id
                });
            }
        }

        Err(HttpResponse::Unauthorized().json("Unauthorized"))
    }

    fn create(credentials: Credentials) -> Result<Self, HttpResponse> {
        let conn = Connection::open("temp/test.db3").unwrap();

        conn.execute_batch(
            r"CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                steamid TEXT,
                gogid TEXT,
                epicid TEXT
            );"
        ).unwrap();

        let hashed_password = hash(&credentials.password, DEFAULT_COST).unwrap();
        conn.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            params![credentials.username, hashed_password],
        ).unwrap();

        let id = conn.last_insert_rowid();

        Ok(User {
            id,
            username: credentials.username,
            password: hashed_password,
            steamid: String::new(),
            gogid: String::new(),
            epicid: String::new()
        })
    }
}

pub async fn register(credentials: web::Json<Credentials>) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match User::create(credentials) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Err(InternalError::from_response("", err).into()),
    }
}

pub async fn login(
    credentials: web::Json<Credentials>,
    session: Session,
) -> Result<impl Responder, Error> {
    let credentials = credentials.into_inner();

    match User::authenticate(credentials) {
        Ok(user) => {
            session.insert("user_id", user.id).unwrap();
            Ok(HttpResponse::Ok().body(format!("Welcome {}!", user.id)))
        }
        Err(err) => Err(InternalError::from_response("", HttpResponse::Unauthorized().body(format!("Authentication error: {:?}", err))).into()),
    }
}


pub async fn index(session: Session) -> Result<HttpResponse> {
    if let Some(user_id) = session.get::<i32>("user_id")? {
        Ok(HttpResponse::Ok().body(format!("Steam ID: {}", user_id)))
    } else {
        Ok(HttpResponse::BadRequest().body("Steam ID not found"))
    }
}

async fn secret(session: Session) -> Result<impl Responder, Error> {
    validate_session(&session).map_err(|err| InternalError::from_response("", err))?;

    Ok("secret revealed")
}

pub fn validate_session(session: &Session) -> Result<i64, HttpResponse> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);

    match user_id {
        Some(id) => {
            session.renew();
            Ok(id)
        }
        None => Err(HttpResponse::Unauthorized().json("Unauthorizedd")),
    }
}
