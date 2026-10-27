use std::path::Path;

use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{hash::hash_image, paths::THUMBNAILS_REL_PATH, scan::scan_images};

pub fn generate_thumbnail_from_path(path: &Path) -> Result<DynamicImage> {
    let img = image::open(path)?;

    Ok(generate_thumbnail(&img))
}

pub fn generate_thumbnail(img: &DynamicImage) -> DynamicImage {
    img.resize(512, 512, FilterType::Lanczos3)
}

pub fn thumbnailing_image(img_path: &Path, thumb_path: &Path) -> Result<()> {
    let img = image::open(img_path).with_context(|| format!("failed open image {:?}", img_path))?;
    let thumbnail = generate_thumbnail(&img);
    let thumbnail_save_path = thumb_path.join(format!("{}.png", hash_image(&img)));

    thumbnail.save(&thumbnail_save_path).with_context(|| {
        format!(
            "failed save thumbnail for {:?} at {:?}",
            img_path, thumbnail_save_path
        )
    })?;

    Ok(())
}

pub fn thumbnailing_folder(root: &Path) -> Result<()> {
    let image_paths = scan_images(root)?;
    let total_images = image_paths.len();

    println!("Thumbnailing {} images...", total_images);
    let thumb_path = root.join(THUMBNAILS_REL_PATH);
    let results: Vec<Result<()>> = image_paths
        .par_iter()
        .map(|img_path| thumbnailing_image(img_path, &thumb_path))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();
    println!("{}/{} done", total_images - err_count, total_images);
    results.iter().take(10).for_each(|result| {
        if let Err(err) = result {
            eprintln!("\n{:#}", err);
        }
    });

    Ok(())
}
