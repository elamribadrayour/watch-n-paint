use anyhow::{Ok, Result};
use image::imageops::FilterType;
use image::{Rgb, RgbImage};
use uuid::Uuid;

pub struct Item {
    pub value: Vec<f64>,
    pub file_path: String,
}

impl Item {
    pub fn frompath(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            ..Default::default()
        }
    }

    pub fn fromvalue(value: Vec<f64>) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn value(&self, width: u32, height: u32) -> Result<Vec<f64>> {
        if !self.value.is_empty() {
            return Ok(self.value.clone());
        }

        let value = image::open(&self.file_path)?
            .resize_exact(width, height, FilterType::Lanczos3)
            .to_rgb8()
            .to_vec()
            .iter()
            .map(|&x| (x as f64) / (255.0))
            .collect::<Vec<f64>>();

        Ok(value)
    }

    pub fn image(&self, width: u32, height: u32) -> Result<image::RgbImage> {
        if self.value.is_empty() {
            return Err(anyhow::Error::msg("Value is empty"));
        }

        let mut img = RgbImage::new(width, height);
        for (i, pixel) in self.value.chunks(3).enumerate() {
            let x = (i as u32) % width;
            let y = (i as u32) / width;
            let r = (pixel[0] * 255.0) as u8;
            let g = (pixel[1] * 255.0) as u8;
            let b = (pixel[2] * 255.0) as u8;
            img.put_pixel(x, y, Rgb([r, g, b]));
        }

        Ok(img)
    }

    pub fn save(&self, width: u32, height: u32) -> Result<()> {
        if self.value.is_empty() {
            return Err(anyhow::Error::msg("Value is empty"));
        }

        let image = self.image(width, height)?;
        image.save(&self.file_path)?;
        Ok(())
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            file_path: format!("{}.jpg", Uuid::new_v4()),
            value: Vec::new(),
        }
    }
}
