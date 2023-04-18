mod routes;

use std::net::SocketAddr;

pub async fn run(addr: SocketAddr) {
    let app = routes::get_router().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
