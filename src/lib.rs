use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum SignupResponse {
    Success,
    Failure(String),
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse {
    Success { token: u128 },
    Failure(String),
}

#[derive(Serialize, Deserialize)]
pub struct LogoutRequest {
    pub token: u128,
    pub all: bool,
}

#[derive(Serialize, Deserialize)]
pub enum LogoutResponse {
    Success,
    Failure(String),
}
