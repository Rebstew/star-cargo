
use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;

pub async fn connect_to_db() -> Result<Database, Box<dyn Error>> {
    println!("connect_to_db -- Connecting to MongoDB...");

    // Set up MongoDB client options. Replace the URI with your MongoDB instance URI.
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Print a warning id the selected database doesn't exist -- in Mongo terms, it means it is empty.
    if !client.list_database_names(None, None).await?.contains(&"star_cargo".to_string()) {
        println!("connect_to_db -- Database 'star_cargo' doesn't exist. You might want to initialise it with data...");
    }

    println!("connect_to_db -- Connected to MongoDB on database 'star_cargo'");

    // Access the specific database (create it if it doesn't exist).
    Ok(client.database("star_cargo"))
}
