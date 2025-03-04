use rusqlite::Connection;
use std::error::Error;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use thiserror::Error;
use regex::Regex;

use crate::models::VitalSigns;
use crate::db::{init_db, log_to_db};

#[derive(Error, Debug)]
pub enum VitalSignsError {
    #[error("Invalid data format: {0}")]
    ParseError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tungstenite::Error),
}

pub async fn run_noah_agent() -> Result<(), Box<dyn Error>> {
    let url = "ws://127.0.0.1:8080/ws";
    let (mut socket, _response) = connect_async(url).await?;

    let conn = Connection::open("vitals.db")?;
    init_db(&conn).await?;

    while let Some(message) = socket.next().await {
        let text = message?.to_text()?.to_string();
        let vitals = parse_vitals(&text)?;

        log_to_db(&conn, &vitals).await?;

        println!(
            "Logged: BP: {}, HR: {}, SpO2: {}",
            vitals.blood_pressure, vitals.heart_rate, vitals.oxygen_saturation
        );
    }

    Ok(())
}

fn parse_vitals(data: &str) -> Result<VitalSigns, Box<dyn Error>> {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 3 {
        return Err("Invalid data format".into());
    }
    let bp = parts[0].trim_start_matches("BP:").trim().to_string();
    let hr: i32 = parts[1].trim_start_matches("HR:").trim().parse()?;
    let spo2: i32 = parts[2].trim_start_matches("SpO2:").trim().parse()?;

    Ok(VitalSigns { blood_pressure: bp, heart_rate: hr, oxygen_saturation: spo2 })
} 