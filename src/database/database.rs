use std::env;
use mongodb::{Client, Database, error::Result};

pub async fn connect_mongodb() -> Result<Database> {
    let db_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");
    let db_name = env::var("MONGODB_DB").expect("MONGODB_DB must be set");
    let client = Client::with_uri_str(db_url).await?;
    let db = client.database(&db_name);
    println!("Connected to MongoDB");
    Ok(db)
}
