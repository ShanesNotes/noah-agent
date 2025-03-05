#![allow(unused_imports)]
#![allow(dead_code)]

use rusqlite::{Connection, Result, ToSql};
use crate::models::VitalSigns;
use thiserror::Error;
use chrono::Local;
use serde_json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database operation failed: {0}")]
    OperationError(#[from] rusqlite::Error),
}

pub fn log_to_db(conn: &Connection, vitals: &VitalSigns) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "INSERT INTO vitals (
            patient_id, heart_rate, blood_pressure_systolic, blood_pressure_diastolic,
            oxygen_saturation, respiratory_rate, temperature
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )?;
    let vitals_id = stmt.insert([
        &vitals.patient_id as &dyn ToSql,
        vitals.heart_rate.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
        vitals.blood_pressure_systolic.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
        vitals.blood_pressure_diastolic.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
        vitals.oxygen_saturation.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
        vitals.respiratory_rate.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
        vitals.temperature.as_ref().map(|v| v as &dyn ToSql).unwrap_or(&rusqlite::types::Null),
    ])?;

    for (key, value) in &vitals.additional_params {
        conn.execute(
            "INSERT INTO additional_params (vitals_id, param_name, param_value) VALUES (?1, ?2, ?3)",
            [&vitals_id as &dyn ToSql, key, value],
        )?;
    }
    Ok(())
}

pub fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vitals (
            id INTEGER PRIMARY KEY,
            patient_id TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            heart_rate INTEGER,
            blood_pressure_systolic INTEGER,
            blood_pressure_diastolic INTEGER,
            oxygen_saturation INTEGER,
            respiratory_rate INTEGER,
            temperature REAL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS additional_params (
            vitals_id INTEGER,
            param_name TEXT,
            param_value REAL,
            FOREIGN KEY(vitals_id) REFERENCES vitals(id)
        )",
        [],
    )?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VitalSignsData {
    pub patient_id: String,
    pub blood_pressure_systolic: Option<i32>,
    pub blood_pressure_diastolic: Option<i32>,
    pub heart_rate: Option<i32>,
    pub oxygen_saturation: Option<i32>,
    pub respiratory_rate: Option<i32>,
    pub temperature: Option<f32>,
    pub central_venous_pressure: Option<i32>,
    pub pulmonary_artery_pressure_systolic: Option<i32>,
    pub pulmonary_artery_pressure_diastolic: Option<i32>,
    pub pulmonary_artery_wedge_pressure: Option<i32>,
    pub cardiac_output: Option<f32>,
    pub systemic_vascular_resistance: Option<i32>,
    pub additional_params: HashMap<String, String>,
}

impl VitalSigns {
    pub fn get_abnormal_vitals(&self) -> Vec<(String, String)> {
        let mut abnormal = Vec::new();
        if let Some(bp_sys) = self.blood_pressure_systolic {
            if bp_sys < 90 || bp_sys > 140 {
                abnormal.push(("Blood Pressure (Systolic)".to_string(), bp_sys.to_string()));
            }
        }
        if let Some(bp_dia) = self.blood_pressure_diastolic {
            if bp_dia < 60 || bp_dia > 90 {
                abnormal.push(("Blood Pressure (Diastolic)".to_string(), bp_dia.to_string()));
            }
        }
        if let Some(hr) = self.heart_rate {
            if hr < 60 || hr > 100 {
                abnormal.push(("Heart Rate".to_string(), hr.to_string()));
            }
        }
        if let Some(o2) = self.oxygen_saturation {
            if o2 < 90 {
                abnormal.push(("Oxygen Saturation".to_string(), o2.to_string()));
            }
        }
        abnormal
    }
} 