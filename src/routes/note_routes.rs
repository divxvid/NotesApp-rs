use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

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

pub async fn get_note_with_id(Path(id): Path<String>) -> String {
    format!("Hello from get note with id: {}", id)
}

pub async fn delete_note_with_id(Path(id): Path<String>) -> String {
    format!("Hello from delete note with id: {}", id)
}

pub async fn get_all_notes() -> &'static str {
    "Hello from get all notes"
}

pub async fn add_note(Json(body): Json<Note>) -> Json<NoteCreateResponse> {
    let resp = NoteCreateResponse {
        message: "Add Note from Axum".to_owned(),
        title: body.title,
        note: body.note,
    };

    Json(resp)
}
