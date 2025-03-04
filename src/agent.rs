use rusqlite::Connection;
use std::error::Error;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use thiserror::Error;
use regex::Regex;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

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
    let url = url::Url::parse("ws://127.0.0.1:8080/ws")?;
    let (mut socket, _) = connect_async(url).await?;

    let conn = Connection::open("vitals.db")?;
    init_db(&conn)?;

    while let Some(message) = socket.next().await {
        let text = message?.into_text()?;
        let vitals = parse_vitals(&text)?;

        log_to_db(&conn, &vitals)?;

        println!(
            "Logged: BP: {}, HR: {}, SpO2: {}",
            vitals.blood_pressure, vitals.heart_rate, vitals.oxygen_saturation
        );
    }

    Ok(())
}

fn parse_vitals(data: &str) -> Result<VitalSigns, VitalSignsError> {
    let re = Regex::new(r"BP:(\d+/\d+),HR:(\d+),SpO2:(\d+)").unwrap();
    let caps = re
        .captures(data)
        .ok_or(VitalSignsError::ParseError("Invalid format".to_string()))?;
    let bp = caps.get(1).unwrap().as_str().to_string();
    let hr: i32 = caps.get(2).unwrap().as_str().parse().map_err(|e| VitalSignsError::ParseError(e.to_string()))?;
    let spo2: i32 = caps.get(3).unwrap().as_str().parse().map_err(|e| VitalSignsError::ParseError(e.to_string()))?;
    Ok(VitalSigns { blood_pressure: bp, heart_rate: hr, oxygen_saturation: spo2 })
}

pub async fn run_websocket_server() {
    let addr = "127.0.0.1:8080";
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |mut socket| async move {
                let mut interval = interval(Duration::from_secs(5));
                let mut rng = StdRng::from_entropy();
                loop {
                    interval.tick().await;
                    let bp_systolic = rng.gen_range(115..=125);
                    let bp_diastolic = rng.gen_range(75..=85);
                    let heart_rate = rng.gen_range(65..=75);
                    let oxygen_saturation = rng.gen_range(95..=100);
                    let message = format!("BP:{}/{}", bp_systolic, bp_diastolic);
                    let message = format!("{},HR:{},SpO2:{}", message, heart_rate, oxygen_saturation);
                    socket.send(Message::text(message)).await.ok();
                }
            })
        });

    println!("WebSocket server running at ws://{}", addr);
    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
} 