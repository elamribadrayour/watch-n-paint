use anyhow::{Ok, Result};

use crate::{initializer, optimizer::Optimizer};

pub struct Neuron {
    pub bias: f64,
    pub weights: Vec<f64>,
}

impl Neuron {
    pub fn new(size: usize, initializer: &mut dyn initializer::Initializer) -> Self {
        Self {
            weights: initializer.initialize(size),
            bias: *initializer.initialize(1).first().unwrap(),
        }
    }

    pub fn forward(&self, inputs: &[f64]) -> Result<f64> {
        if self.weights.len() != inputs.len() {
            return Err(anyhow::anyhow!("weights & inputs vectors don't have the same length"))
        }

        let mut output = 0.0;
        for (&x, &w) in inputs.iter().zip(self.weights.iter()) {
            output += x * w;
        }
        output += self.bias;
        Ok(output)
    }

    pub fn backward(&mut self, inputs: &[f64], output_grad: f64, optimizer: &mut dyn Optimizer) -> Result<Vec<f64>> {
        let bias_gradient = output_grad;
        let weight_gradients = inputs.iter().map(|&x| x * output_grad).collect::<Vec<f64>>();

        optimizer.optimize(&mut self.weights, &weight_gradients)?;

        let mut bias = vec![self.bias];
        optimizer.optimize(&mut bias, &vec![bias_gradient])?;
        self.bias = bias[0];

        Ok(weight_gradients)
    }
}
