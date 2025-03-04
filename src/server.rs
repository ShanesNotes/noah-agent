use std::time::Duration;
use futures_util::SinkExt;
use warp::Filter;
use warp::ws::Message;
use tokio::time::interval;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

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
                    let _ = socket.send(Message::text("BP:120/80,HR:70,SpO2:98")).await;
                }
            })
        });

    println!("WebSocket server running at ws://{}", addr);
    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
} 