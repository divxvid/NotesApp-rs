use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{data_models::UserPass, server_state::ServerState};

#[derive(Deserialize)]
pub struct UserInformation {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    message: String,
    username: String,
}

// Make sure the order of extractors is okay
// link: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
pub async fn handle_signup(
    State(state): State<ServerState>,
    Json(body): Json<UserInformation>,
) -> Result<Json<UserResponse>, StatusCode> {
    let collection = state.db.collection::<UserPass>("userpasses");
    let new_entry = UserPass {
        username: body.username.clone(),
        password: body.password,
    };
    let db_result = collection.insert_one(new_entry, None).await;

    match db_result {
        Ok(insert_result) => {
            let id = insert_result.inserted_id;
            println!("Record Inserted with ID: {}", id);
            let resp = UserResponse {
                message: format!("Created new user in DB with ID: {}", id),
                username: body.username,
            };
            Ok(Json(resp))
        }
        Err(err) => {
            eprintln!("An Error Occured at handle_signup:\n {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_login(Json(body): Json<UserInformation>) -> Json<UserResponse> {
    let resp = UserResponse {
        message: "Login Stub Message from Axum".to_owned(),
        username: body.username,
    };

    Json(resp)
}
