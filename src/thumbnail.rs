use std::path::Path;

use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};

pub fn generate_thumbnail_from_path<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
    let img = image::open(path)?;

    Ok(generate_thumbnail(&img))
}

pub fn generate_thumbnail(img: &DynamicImage) -> DynamicImage {
    img.resize(512, 512, FilterType::Lanczos3)
}
