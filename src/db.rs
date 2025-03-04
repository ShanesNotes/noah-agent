use rusqlite::Connection;
use crate::models::VitalSigns;
use serde::{Serialize, Deserialize};

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