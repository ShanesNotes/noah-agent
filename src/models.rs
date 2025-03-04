use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VitalSigns {
    pub blood_pressure: String,  // e.g., "120/80"
    pub heart_rate: i32,         // e.g., 70
    pub oxygen_saturation: i32,  // e.g., 98
} 