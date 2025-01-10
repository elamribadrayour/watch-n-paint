use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Optimizer {
    pub lr: f64,
    pub name: String,
}
