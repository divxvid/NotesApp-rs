use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::{auth::JWTClaims, data_models::NoteModel, server_state::ServerState};

#[derive(Serialize, Deserialize)]
pub struct Note {
    title: String,
    note: String,
}

#[derive(Serialize)]
pub struct NoteCreateResponse {
    message: String,
    title: String,
    note: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteFromDB {
    _id: ObjectId,
    username: String,
    title: String,
    note: String,
}

#[derive(Serialize)]
pub struct NoteDTO {
    _id: String,
    username: String,
    title: String,
    note: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

pub async fn get_note_with_id(
    Path(id): Path<String>,
    State(state): State<ServerState>,
) -> Result<Json<NoteDTO>, StatusCode> {
    let collection = state.db.collection::<NoteFromDB>("notes");
    let id = ObjectId::from_str(&id).unwrap();
    let note = collection
        .find_one(doc! { "_id": id }, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(note) = note {
        Ok(Json(NoteDTO {
            _id: note._id.to_string(),
            username: note.username,
            title: note.title,
            note: note.note,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_note_with_id(
    Path(id): Path<String>,
    State(state): State<ServerState>,
) -> Result<StatusCode, StatusCode> {
    let collection = state.db.collection::<NoteFromDB>("notes");
    let id = ObjectId::from_str(&id).unwrap();
    collection
        .delete_one(doc! { "_id": id }, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn get_all_notes(
    Extension(claims): Extension<JWTClaims>,
    State(state): State<ServerState>,
) -> Result<Json<Vec<NoteDTO>>, StatusCode> {
    let collection = state.db.collection::<NoteFromDB>("notes");
    let mut cursor = collection
        .find(doc! { "username": claims.username }, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut notes = vec![];
    while let Some(note) = cursor
        .try_next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        notes.push(NoteDTO {
            _id: note._id.to_string(),
            username: note.username,
            title: note.title,
            note: note.note,
        });
    }

    Ok(Json(notes))
}

pub async fn add_note(
    State(state): State<ServerState>,
    Extension(claims): Extension<JWTClaims>,
    Json(body): Json<Note>,
) -> Result<StatusCode, (StatusCode, Json<ErrorMessage>)> {
    body.validate()
        .map_err(|err| (StatusCode::BAD_REQUEST, Json(ErrorMessage { message: err })))?;

    let collection = state.db.collection::<NoteModel>("notes");
    let new_note = NoteModel {
        username: claims.username,
        title: body.title,
        note: body.note,
    };

    collection.insert_one(new_note, None).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: "could not insert note!".to_owned(),
            }),
        )
    })?;

    Ok(StatusCode::CREATED)
}

impl Note {
    fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            Err("Title cannot be empty".to_owned())
        } else {
            Ok(())
        }
    }
}
