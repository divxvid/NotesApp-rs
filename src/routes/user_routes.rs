use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
}

// Make sure the order of extractors is okay
// link: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
pub async fn handle_signup(
    State(state): State<ServerState>,
    Json(body): Json<UserInformation>,
) -> impl IntoResponse {
    body.validate()
        .map_err(|message| (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })))?;

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
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Error encountered while creating user in db".to_owned(),
                }),
            ))
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

impl UserInformation {
    fn validate(&self) -> Result<(), String> {
        let error_messages = [self.validate_username(), self.validate_password()]
            .iter()
            .cloned()
            .filter_map(|x| x.err())
            .collect::<Vec<String>>();

        match error_messages.is_empty() {
            true => Ok(()),
            false => Err(error_messages.join(", ")),
        }
    }

    fn validate_username(&self) -> Result<(), String> {
        if self.username.is_empty() {
            return Err("Username cannot be Empty".to_owned());
        }
        Ok(())
    }

    fn validate_password(&self) -> Result<(), String> {
        if self.password.is_empty() {
            return Err("Password cannot be Empty".to_owned());
        }
        Ok(())
    }
}
