use rand::Rng;
use rand::RngCore;

use crate::config;
use crate::initializer;


pub struct ZeroCentered {
    min: f64,
    max: f64,
    rng: Box<dyn RngCore>,
}

impl ZeroCentered {
    pub fn new(config: &config::Initializer) -> Self {
        Self {
            max: config.radius,
            min: -config.radius,
            rng: Box::new(rand::thread_rng())
        }
    }
}

impl initializer::Initializer for ZeroCentered {
    fn initialize(&mut self, size: usize) -> Vec<f64> {
        (0..size).map(|_| self.rng.gen_range(self.min..self.max)).collect()
    }
}