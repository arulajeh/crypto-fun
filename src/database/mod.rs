pub mod collections;

use std::env;
use mongodb::{Client, Database, error::Result};

pub async fn connect_mongodb() -> Result<Database> {
    let db_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let db_name = env::var("MONGODB_DB").expect("MONGODB_DB must be set");
    let mut client_options = mongodb::options::ClientOptions::parse(&db_url).await?;
    client_options.max_pool_size = Some(20);
    client_options.min_pool_size = Some(5);
    client_options.server_selection_timeout = Some(std::time::Duration::from_millis(3000));
    client_options.connect_timeout = Some(std::time::Duration::from_millis(5000));
    let client = Client::with_options(client_options)?;
    let db = client.database(&db_name);
    println!("Connected to MongoDB");
    Ok(db)
}