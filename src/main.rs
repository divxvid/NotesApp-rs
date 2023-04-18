use std::net::SocketAddr;

use notes_app_rs::run;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(addr).await;
}
