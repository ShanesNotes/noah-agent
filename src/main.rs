use std::error::Error;

mod server;
mod agent;
mod models;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run the simulated Philips monitor server in the background
    let server_handle = tokio::spawn(server::run_websocket_server());
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Run the Noah agent
    agent::run_noah_agent().await?;

    // Wait for the server task to finish (though it's infinite)
    server_handle.await.unwrap();

    Ok(())
}
