mod auth;
mod data_models;
mod routes;
mod server_state;

use std::{error::Error, net::SocketAddr};

pub async fn run(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let app = routes::get_router().await?;

    tracing::info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
