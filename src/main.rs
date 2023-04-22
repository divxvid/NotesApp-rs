use std::{error::Error, net::SocketAddr};

use dotenv;
use futures::stream::TryStreamExt;
use models::Note;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use notes_app_rs::run;

use crate::models::UserPass;

mod models;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run(addr).await;
    test_db().await.expect("Some Error occured in client");
}

async fn test_db() -> Result<(), Box<dyn Error>> {
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

    // println!("Databases:");
    // for name in client.list_database_names(None, None).await? {
    //     println!("- {}", name);
    // }
    let db = client.database("test");
    // println!("Collections:");
    // for name in db.list_collection_names(None).await? {
    //     println!("- {}", name);
    // }

    let collection = db.collection::<UserPass>("userpasses");
    let mut cursor = collection.find(None, None).await?;

    while let Some(data) = cursor.try_next().await? {
        dbg!(data);
    }

    let collection = db.collection::<Note>("notes");
    let mut cursor = collection.find(None, None).await?;

    while let Some(data) = cursor.try_next().await? {
        dbg!(data);
    }

    // println!("done deleted count: {}", r.deleted_count);
    Ok(())
}
