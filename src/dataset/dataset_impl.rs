use anyhow::{Ok, Result};

use crate::config;
use crate::dataset::Item;

pub struct Dataset {
    pub width: u32,
    pub height: u32,
    pub items: Vec<Item>,
}

impl Dataset {
    pub fn new(config: &config::Dataset) -> Result<Self> {
        Ok(Self {
            width: config.width,
            height: config.height,
            items: Dataset::get_items(&config.path)?,
        })
    }

    pub fn get_items(dir_path: &str) -> Result<Vec<Item>> {
        let items = std::fs::read_dir(dir_path)?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let path_str = path.to_str()?;
                Some(Item::frompath(path_str))
            })
            .collect::<Vec<Item>>();

        Ok(items)
    }
}
