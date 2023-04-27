use axum::{
    extract::State,
    http::{header::SET_COOKIE, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::Cookie;
use bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::{auth::get_token, data_models::UserPass, server_state::ServerState};

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
    let collection = state.db.collection::<UserPass>("userpasses");
    validate_signup_request(&body, &collection).await?;

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

pub async fn handle_login(
    State(state): State<ServerState>,
    Json(body): Json<UserInformation>,
) -> impl IntoResponse {
    let collection = state.db.collection::<UserPass>("userpasses");

    let db_result = collection
        .find_one(doc! { "username": body.username.clone() }, None)
        .await
        .map_err(|err| {
            eprint!("Error encountered in reading Collection userpasses from /login\n{err}",);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Error occured while reading from db".to_owned(),
                }),
            );
        })?;

    let db_result = db_result.unwrap_or(UserPass::default());
    if body.password != db_result.password {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Invalid Credentials!".to_owned(),
            }),
        ));
    }

    let jwt_token = get_token(&body.username).map_err(|err| {
        eprintln!("Error occured at JWT creation.\n{err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Error generating JWT token".to_owned(),
            }),
        );
    })?;

    let cookie = Cookie::build("token", jwt_token)
        .path("/")
        .max_age(Duration::hours(1))
        .http_only(true)
        .finish();

    // let resp = UserResponse {
    //     message: "Login Stub Message from Axum".to_owned(),
    //     username: body.username,
    // };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(SET_COOKIE, cookie.to_string())
        .body("Login Successful!".to_string())
        .unwrap();

    Ok(response)
}

async fn validate_signup_request(
    body: &UserInformation,
    collection: &Collection<UserPass>,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    body.validate()
        .map_err(|message| (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })))?;

    let res = collection
        .find_one(doc! { "username": body.username.clone() }, None)
        .await;
    match res {
        Ok(x) => match x {
            Some(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        message: format!("Username: {} already exists!", body.username),
                    }),
                ))
            }
            None => Ok(()),
        },
        Err(err) => {
            eprintln!("An Error Occured at handle_signup:\n {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Error encountered while checking username in db".to_owned(),
                }),
            ));
        }
    }
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
