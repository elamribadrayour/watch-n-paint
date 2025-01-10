use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub width: u32,
    pub height: u32,
    pub path: String,
}
