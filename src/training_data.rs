use std::collections::HashMap;
use crate::models::VitalSigns;

pub struct TrainingData {
    vital_ranges: HashMap<String, (f64, f64, f64, f64)>, // min, max, normal_min, normal_max
}

impl TrainingData {
    pub fn new() -> Self {
        let mut vital_ranges = HashMap::new();
        vital_ranges.insert("heart_rate".to_string(), (30.0, 200.0, 60.0, 100.0));
        vital_ranges.insert("blood_pressure_systolic".to_string(), (60.0, 200.0, 90.0, 140.0));
        vital_ranges.insert("blood_pressure_diastolic".to_string(), (40.0, 120.0, 60.0, 90.0));
        vital_ranges.insert("oxygen_saturation".to_string(), (70.0, 100.0, 90.0, 100.0));
        TrainingData { vital_ranges }
    }

    pub fn validate(&self, vitals: &VitalSigns) -> Result<(), String> {
        let mut errors = Vec::new();
        let default_range = (0.0, f64::MAX, 0.0, f64::MAX); // Default if key missing

        if let Some(hr) = vitals.heart_rate {
            let (min, max, _, _) = self.vital_ranges.get("heart_rate").unwrap_or(&default_range);
            if (hr as f64) < *min || (hr as f64) > *max {
                errors.push(format!("Heart rate {} out of range ({}-{})", hr, min, max));
            }
        }
        if let Some(sys) = vitals.blood_pressure_systolic {
            let (min, max, _, _) = self.vital_ranges.get("blood_pressure_systolic").unwrap_or(&default_range);
            if (sys as f64) < *min || (sys as f64) > *max {
                errors.push(format!("Systolic BP {} out of range ({}-{})", sys, min, max));
            }
        }
        if let Some(dia) = vitals.blood_pressure_diastolic {
            let (min, max, _, _) = self.vital_ranges.get("blood_pressure_diastolic").unwrap_or(&default_range);
            if (dia as f64) < *min || (dia as f64) > *max {
                errors.push(format!("Diastolic BP {} out of range ({}-{})", dia, min, max));
            }
        }
        if let Some(o2) = vitals.oxygen_saturation {
            let (min, max, _, _) = self.vital_ranges.get("oxygen_saturation").unwrap_or(&default_range);
            if (o2 as f64) < *min || (o2 as f64) > *max {
                errors.push(format!("O2 saturation {} out of range ({}-{})", o2, min, max));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }

    pub async fn get_recommendation(&self, vitals: &VitalSigns) -> String {
        if let Some(o2) = vitals.oxygen_saturation {
            if o2 < 90 {
                return "Consider supplemental oxygen.".to_string();
            }
        }
        "No immediate action required.".to_string()
    }
}