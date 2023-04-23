use std::{error::Error, net::SocketAddr};

use dotenv;
use notes_app_rs::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(addr).await?;

    Ok(())
}
