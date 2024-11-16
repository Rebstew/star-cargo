use mongodb::{Client, Database, options::ClientOptions};
use std::error::Error;

pub async fn connect_to_db() -> Result<Database, Box<dyn Error>> {
    // Set up MongoDB client options. Replace the URI with your MongoDB instance URI.
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Access the specific database (create it if it doesn't exist).
    Ok(client.database("star_cargo"))
}
