use anyhow::Result;
use config::Config;
use dataset::{Dataset, Item};

mod config;
mod dataset;

fn main() -> Result<()> {
    let config = Config::new("Config.json")?;

    let dataset = Dataset::new(&config.dataset)?;
    println!(
        "size: {}, width: {} -- height: {}",
        dataset.items.len(),
        dataset.width,
        dataset.height
    );

    let item = dataset.items.first().unwrap();
    let value = item.value(dataset.width, dataset.height)?;
    println!("first item path {}", item.file_path);
    println!("first item value length {:?}", value.len());

    let item = Item::fromvalue(value);
    let image = item.image(dataset.width, dataset.height)?;
    item.save(dataset.width, dataset.height)?;
    println!(
        "rebuild image dims {:?}x{:?}",
        image.width(),
        image.height()
    );
    Ok(())
}
