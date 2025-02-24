use std::{collections::HashMap, sync::Arc};

use argon2::{
    password_hash::{PasswordHashString, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use chrono::{DateTime, Days, Utc};
use log::{info, warn};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Whatever};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct Session {
    expiration: DateTime<Utc>,
    username: String,
}

struct AppState {
    salt_rng: OsRng,
    token_rng: OsRng,
    users: HashMap<String, PasswordHashString>,
    sessions: HashMap<u128, Session>,
}

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
enum SignupResponse {
    Success,
    Failure(String),
}

//#[axum::debug_handler]
async fn signup(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<SignupRequest>,
) -> Json<SignupResponse> {
    let mut state = state.lock().await;
    #[allow(clippy::map_entry)]
    if state.users.contains_key(&request.username) {
        warn!("Username already exists");
        Json(SignupResponse::Failure(
            "sucks to suck username taken".into(),
        ))
    } else {
        info!("Adding account");
        let salt = SaltString::generate(&mut state.salt_rng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)
            .expect("bro....")
            .into();

        state.users.insert(request.username, password_hash);

        Json(SignupResponse::Success)
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
enum LoginResponse {
    Success { token: u128 },
    Failure(String),
}

async fn login(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<SignupRequest>,
) -> Json<LoginResponse> {
    let mut state = state.lock().await;

    let Some(password_hash) = state.users.get(&request.username) else {
        return Json(LoginResponse::Failure(
            "invalid username or password".into(),
        ));
    };

    if Argon2::default()
        .verify_password(request.password.as_bytes(), &password_hash.password_hash())
        .is_err()
    {
        return Json(LoginResponse::Failure(
            "invalid username or password".into(),
        ));
    }

    let mut token_bytes = [0; 16];
    state.token_rng.fill_bytes(&mut token_bytes);
    let token = u128::from_ne_bytes(token_bytes);

    let expiration = Utc::now()
        .checked_add_days(Days::new(30))
        .expect("failed to create expiration date");

    state.sessions.insert(
        token,
        Session {
            expiration,
            username: request.username,
        },
    );

    info!("current sessions: {:?}", state.sessions);

    Json(LoginResponse::Success { token })
}

#[derive(Deserialize)]
struct LogoutRequest {
    token: u128,
    all: bool,
}

#[derive(Serialize)]
enum LogoutResponse {
    Success,
    Failure(String),
}

async fn logout(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<LogoutRequest>,
) -> Json<LogoutResponse> {
    let mut state = state.lock().await;

    let Some(username) = state
        .sessions
        .get(&request.token)
        .map(|session| &session.username)
        .cloned()
    else {
        return Json(LogoutResponse::Failure("Invalid token".into()));
    };

    if request.all {
        state
            .sessions
            .retain(|_, session| session.username != username);
    } else {
        state.sessions.remove(&request.token);
    }

    info!("current sessions: {:?}", state.sessions);

    Json(LogoutResponse::Success)
}

async fn publish() {
    todo!()
}

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), Whatever> {
    env_logger::init();

    let shared_state = Arc::new(Mutex::new(AppState {
        salt_rng: OsRng,
        token_rng: OsRng,
        users: HashMap::default(),
        sessions: HashMap::default(),
    }));

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/publish", post(publish))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .whatever_context("Failed to serve")
}
