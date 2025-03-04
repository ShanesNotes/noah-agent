use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VitalSigns {
    pub blood_pressure: String,
    pub heart_rate: i32,
    pub oxygen_saturation: i32,
} 