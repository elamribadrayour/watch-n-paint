use anyhow::Result;

use crate::config;
use crate::initializer::ZeroCentered;

pub trait Initializer {
    fn initialize(&mut self, size: usize) -> Vec<f64>;
}

pub fn get_initializer(config: &config::Initializer) -> Result<Box<dyn Initializer>> {
    match config.name.as_str() {
        "zero-centered" => Ok(Box::new(ZeroCentered::new(config))),
        _ => Err(anyhow::anyhow!("unknown initializer name. choose between zero-centered"))
    }
}