// use std::error::Error;
#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Duration;
use rusqlite::Connection;

mod server;
mod agent;
mod models;
mod db;
mod training_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Run the simulated Philips monitor server in the background
    let server_handle = tokio::spawn(server::run_websocket_server());
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Open the database connection and pass it to run_noah_agent
    let conn = Connection::open("vitals.db")?;
    db::init_db(&conn)?; // Initialize the database
    agent::run_noah_agent(&conn).await?;

    // Wait for the server task to finish (though it's infinite)
    if let Err(e) = server_handle.await {
        eprintln!("Server task failed: {:?}", e);
    }

    Ok(())
}
