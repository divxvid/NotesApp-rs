use axum::extract::Path;

pub async fn get_note_with_id(Path(id): Path<String>) -> String {
    format!("Hello from get note with id: {}", id)
}

pub async fn delete_note_with_id(Path(id): Path<String>) -> String {
    format!("Hello from delete note with id: {}", id)
}

pub async fn get_all_notes() -> &'static str {
    "Hello from get all notes"
}

pub async fn add_note() -> &'static str {
    "Hello from add note!"
}
