use image::{DynamicImage, imageops::FilterType};
use std::path::Path;

pub fn generate_thumbnail_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<DynamicImage> {
    let img = image::open(path)?;

    Ok(generate_thumbnail(img))
}

pub fn generate_thumbnail(img: image::DynamicImage) -> DynamicImage {
    img.resize(512, 512, FilterType::Lanczos3)
}
