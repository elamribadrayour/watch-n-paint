use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Initializer {
    pub radius: f64,
    pub name: String,
}
