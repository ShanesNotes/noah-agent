use rusqlite::Connection;
use crate::models::VitalSigns;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use futures_util::{StreamExt, SinkExt};
use tungstenite::Message;
use warp::Filter;
use rand::Rng;
use tokio::time::interval;

pub async fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vitals (
            id INTEGER PRIMARY KEY,
            blood_pressure TEXT,
            heart_rate INTEGER,
            oxygen_saturation INTEGER
        )",
        [],
    )?;
    Ok(())
}

pub async fn log_to_db(conn: &Connection, vitals: &VitalSigns) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO vitals (blood_pressure, heart_rate, oxygen_saturation) VALUES (?1, ?2, ?3)",
        [vitals.blood_pressure.as_str(), &vitals.heart_rate.to_string(), &vitals.oxygen_saturation.to_string()],
    )?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VitalSigns {
    pub blood_pressure: String,  // e.g., "120/80"
    pub heart_rate: i32,         // e.g., 70
    pub oxygen_saturation: i32,  // e.g., 98
}

pub async fn run_websocket_server() {
    let addr = "127.0.0.1:8080";
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| async move {
                let mut interval = interval(Duration::from_secs(5));
                let mut rng = rand::thread_rng();
                loop {
                    interval.tick().await;
                    // Generate random vital sign data within normal limits
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