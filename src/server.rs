use std::time::Duration;
use futures_util::{StreamExt, SinkExt};
use tungstenite::Message;
use warp::Filter;
use rand::Rng;
use tokio::time::interval;

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