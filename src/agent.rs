// Add attributes to ignore warnings for future code
#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;
use serde_json;
use rusqlite::Connection;
use thiserror::Error;
use crate::models::VitalSigns;
use crate::db::log_to_db;
use crate::training_data::TrainingData;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tungstenite::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub async fn run_noah_agent(conn: &Connection) -> Result<(), AgentError> {
    let url = Url::parse("ws://127.0.0.1:8080/ws")?;
    let (mut socket, _) = connect_async(url).await?;
    let training_data = TrainingData::new();

    while let Some(message) = socket.next().await {
        let text = message?.into_text().map_err(|e| AgentError::ParseError(e.to_string()))?;
        let vitals: VitalSigns = serde_json::from_str(&text)
            .map_err(|e| AgentError::ParseError(format!("Failed to parse JSON: {}", e)))?;

        vitals.validate().map_err(AgentError::ValidationError)?;
        log_to_db(conn, &vitals)?;
        let abnormal = vitals.get_abnormal_vitals();
        if !abnormal.is_empty() {
            let abnormal_str = abnormal.iter()
                .map(|(name, value)| format!("\t{}: {}", name, value))
                .collect::<Vec<_>>()
                .join("\n");
            println!("Abnormal Vital Signs Detected:\n{}", abnormal_str);
            let recommendation = training_data.get_recommendation(&vitals).await;
            println!("AI Recommendation: {}", recommendation);
        }
    }
    Ok(())
}

// TODO: Integrate RIG for AI-driven recommendations
// TODO: Expand to handle additional healthcare data (labs, assessments, medications, etc.).
// TODO: Replace database logging with FHIR-compliant API calls for Epic EHR integration.