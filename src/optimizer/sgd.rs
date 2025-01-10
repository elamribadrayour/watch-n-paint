use anyhow::Result;

use crate::config;
use crate::optimizer::Optimizer;


pub struct Sgd {
    lr: f64,   
}

impl Sgd {
    pub fn new(config: &config::Optimizer) -> Self {
        Self {
            lr: config.lr
        }
    }
}

impl Optimizer for Sgd {
    fn optimize(&self, weights: &mut [f64], grads: &[f64]) -> Result<()> {

        if weights.len() != grads.len() {
            return Err(anyhow::anyhow!("Weights and gradients must have the same length"));
        }

        for (w, g) in weights.iter_mut().zip(grads.iter()) {
            *w -= g * self.lr;
        }

        Ok(())
    }
}