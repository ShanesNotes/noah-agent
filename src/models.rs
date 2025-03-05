// Add attributes to ignore warnings for future code
#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VitalSigns {
    pub patient_id: String,
    // Core vital signs
    pub heart_rate: Option<i32>,          // bpm
    pub blood_pressure_systolic: Option<i32>, // mmHg
    pub blood_pressure_diastolic: Option<i32>, // mmHg
    pub oxygen_saturation: Option<i32>,   // %
    pub respiratory_rate: Option<i32>,    // breaths/min
    pub temperature: Option<f32>,         // °C
    // Dynamic parameters (e.g., CVP, PAP)
    pub additional_params: HashMap<String, f64>,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Heart rate is out of realistic range.")]
    HeartRateOutOfRange,
    #[error("Oxygen saturation is out of realistic range.")]
    OxygenSaturationOutOfRange,
    #[error("Blood pressure format is invalid.")]
    InvalidBloodPressureFormat,
}

impl VitalSigns {
    /// Validates core vital signs against clinically plausible ranges.
    pub fn validate(&self) -> Result<(), String> {
        let mut errors = Vec::new();
        if let Some(hr) = self.heart_rate {
            if hr < 30 || hr > 200 {
                errors.push(format!("Heart rate {} bpm out of range (30-200)", hr));
            }
        }
        if let Some(bp_sys) = self.blood_pressure_systolic {
            if bp_sys < 60 || bp_sys > 200 {
                errors.push(format!("Systolic BP {} mmHg out of range (60-200)", bp_sys));
            }
        }
        if let Some(bp_dia) = self.blood_pressure_diastolic {
            if bp_dia < 40 || bp_dia > 120 {
                errors.push(format!("Diastolic BP {} mmHg out of range (40-120)", bp_dia));
            }
        }
        if let Some(o2) = self.oxygen_saturation {
            if o2 < 70 || o2 > 100 {
                errors.push(format!("Oxygen saturation {}% out of range (70-100)", o2));
            }
        }
        if let Some(rr) = self.respiratory_rate {
            if rr < 5 || rr > 50 {
                errors.push(format!("Respiratory rate {} breaths/min out of range (5-50)", rr));
            }
        }
        if let Some(temp) = self.temperature {
            if temp < 35.0 || temp > 40.0 {
                errors.push(format!("Temperature {}°C out of range (35.0-40.0)", temp));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}