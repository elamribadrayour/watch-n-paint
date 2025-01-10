mod nn;
mod config;
mod dataset;
mod optimizer;
mod initializer;

use nn::Neuron;
use anyhow::Result;
use config::Config;
use dataset::Dataset;



fn main() -> Result<()> {
    let config = Config::new("Config.json")?;

    let dataset = Dataset::new(&config.dataset)?;
    let mut optimizer = optimizer::get_optimizer(&config.optimizer)?;
    let mut initializer = initializer::get_initializer(&config.initializer)?;


    let size = 10;
    let inputs = initializer.initialize(10);
    let neuron = Neuron::new(size, &mut *initializer);
    let output = neuron.forward(&inputs)?;
    println!("{:?}", output);
    Ok(())
}
