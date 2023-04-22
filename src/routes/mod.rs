mod note_routes;
mod root_route;
mod user_routes;

use std::error::Error;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use tower_http::cors::{Any, CorsLayer};

use note_routes::*;
use root_route::root_route;
use user_routes::*;

use crate::server_state::ServerState;

pub async fn construct_state() -> Result<ServerState, Box<dyn Error>> {
    let mongo_username = std::env::var("MONGO_USER").expect("Mongo User not found in env vars");
    let mongo_password =
        std::env::var("MONGO_PASSWORD").expect("Mongo Password not found in env vars");
    let mongo_cluster =
        std::env::var("MONGO_CLUSTER").expect("Mongo Cluster not found in env vars");

    let mongo_uri = format!(
        "mongodb+srv://{}:{}@{}",
        mongo_username, mongo_password, mongo_cluster
    );

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;
    let db = client.database("test");

    Ok(ServerState { db })
}

pub async fn get_router() -> Result<Router, Box<dyn Error>> {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any);

    let state = construct_state().await?;

    let router = Router::new()
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
        .with_state(state);

    Ok(router)
}
