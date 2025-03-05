// Add attributes to ignore warnings for future code
#![allow(unused_imports)]
#![allow(dead_code)]

use futures_util::sink::SinkExt;
use std::time::Duration;
use warp::Filter;
use warp::ws::Message;
use tokio::time::interval;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use thiserror::Error;
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::signal;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to send WebSocket message: {0}")]
    SendError(#[from] warp::Error),
}

pub async fn run_websocket_server() -> Result<(), Box<dyn std::error::Error + Send + 'static>> {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |mut socket| async move {
                let mut interval = interval(Duration::from_secs(5));
                let mut rng = StdRng::from_entropy();
                println!("WebSocket connection established for client");

                loop {
                    interval.tick().await;
                    let mut additional_params = HashMap::new();
                    additional_params.insert("CVP".to_string(), rng.gen_range(0.0..15.0));
                    additional_params.insert("PAP_systolic".to_string(), rng.gen_range(15.0..30.0));
                    let vitals = json!({
                        "patient_id": "patient1",
                        "heart_rate": rng.gen_range(40..120),
                        "blood_pressure_systolic": rng.gen_range(90..160),
                        "blood_pressure_diastolic": rng.gen_range(50..100),
                        "oxygen_saturation": rng.gen_range(85..100),
                        "respiratory_rate": rng.gen_range(8..30),
                        "temperature": rng.gen_range(36.0..38.5),
                        "additional_params": additional_params,
                    });

                    // Send message with error handling
                    if let Err(e) = socket.send(Message::text(vitals.to_string())).await {
                        eprintln!("Error sending message: {}. Continuing operation...", e);
                        // Optionally, break the loop if the error is critical (e.g., client disconnected)
                        if e.to_string().contains("disconnected") {
                            println!("Client disconnected, closing connection.");
                            break;
                        }
                    }
                }
            })
        });

    // Server startup confirmation
    println!("Starting WebSocket server at ws://127.0.0.1:8080...");

    // Run the server with graceful shutdown
    let (addr, server) = warp::serve(ws_route)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
            signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
            println!("Shutdown signal received, stopping server...");
        });

    // Confirm server is running
    println!("WebSocket server successfully started at ws://{}", addr);

    // Await server completion
    server.await;
    println!("Server stopped.");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VitalSigns {
    pub patient_id: String,
    pub blood_pressure_systolic: Option<i32>,
    pub blood_pressure_diastolic: Option<i32>,
    pub blood_pressure_mean: Option<i32>,
    pub heart_rate: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub temperature: Option<f32>,
    pub oxygen_saturation: Option<i32>,
    pub central_venous_pressure: Option<i32>,
    pub pulmonary_artery_pressure_systolic: Option<i32>,
    pub pulmonary_artery_pressure_diastolic: Option<i32>,
    pub pulmonary_artery_wedge_pressure: Option<i32>,
    pub cardiac_output: Option<f32>,
    pub systemic_vascular_resistance: Option<i32>,
    pub additional_params: HashMap<String, String>,
} 