use std::{error::Error, net::SocketAddr};

use dotenv;
use notes_app_rs::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    run(addr).await?;

    Ok(())
}
