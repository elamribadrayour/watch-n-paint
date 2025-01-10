use anyhow::Result;

use crate::config;
use crate::optimizer::Sgd;

pub trait Optimizer {
    fn optimize(&self, weights: &mut [f64], grads: &[f64]) -> Result<()>;
}

pub fn get_optimizer(config: &config::Optimizer) -> Result<Box<dyn Optimizer>> {
    match config.name.as_str() {
        "sgd" => Ok(Box::new(Sgd::new(config))),
        _ => Err(anyhow::anyhow!("unknown optimizer name. choose between sgd"))
    }
}