use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserInformation {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    message: String,
    username: String,
    password: String,
}

pub async fn handle_signup(Json(body): Json<UserInformation>) -> Json<UserResponse> {
    let resp = UserResponse {
        message: "Signup Stub Message from Axum".to_owned(),
        username: body.username,
        password: body.password,
    };

    Json(resp)
}

pub async fn handle_login(Json(body): Json<UserInformation>) -> Json<UserResponse> {
    let resp = UserResponse {
        message: "Login Stub Message from Axum".to_owned(),
        username: body.username,
        password: body.password,
    };

    Json(resp)
}
