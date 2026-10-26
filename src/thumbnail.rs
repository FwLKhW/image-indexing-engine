use image::{DynamicImage, imageops::FilterType};
use std::path::Path;

pub fn generate_512px_thumbnail<P: AsRef<Path>>(path: P) -> anyhow::Result<DynamicImage> {
    let img = image::open(path)?;

    let thumbnail = img.resize(512, 512, FilterType::Lanczos3);

    Ok(thumbnail)
}
