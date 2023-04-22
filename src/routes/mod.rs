mod note_routes;
mod root_route;
mod user_routes;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use note_routes::*;
use root_route::root_route;
use user_routes::*;

pub async fn get_router() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root_route))
        .route("/signup", post(handle_signup))
        .route("/login", post(handle_login))
        .route(
            "/notes/:id",
            get(get_note_with_id).delete(delete_note_with_id),
        )
        .route("/notes", get(get_all_notes))
        .route("/notes", post(add_note))
        .layer(cors_layer)
}
