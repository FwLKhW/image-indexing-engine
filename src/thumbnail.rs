use std::{fs, path::Path};

use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{hash::blake3_hash, paths::THUMBNAILS_REL_PATH, scan::scan_images};

#[must_use]
pub fn generate_thumbnail(img: &DynamicImage) -> DynamicImage {
    img.resize(512, 512, FilterType::Lanczos3)
}

/// # Errors
///
/// Failed if image not exists
pub fn thumbnailing_image(img_path: &Path, thumb_path: &Path) -> Result<()> {
    let img = image::open(img_path)
        .with_context(|| format!("failed open image {}", img_path.display()))?;
    let thumbnail = generate_thumbnail(&img);
    let thumbnail_save_path = thumb_path.join(format!("{}.png", blake3_hash(&img)));

    thumbnail.save(&thumbnail_save_path).with_context(|| {
        format!(
            "failed save thumbnail for {} at {}",
            img_path.display(),
            thumbnail_save_path.display()
        )
    })?;

    Ok(())
}

/// # Errors
///
/// Fail if can not create thumbnails folder
pub fn thumbnailing_folder(root: &Path) -> Result<()> {
    let image_paths = scan_images(root);
    let total_images = image_paths.len();

    let thumb_path = root.join(THUMBNAILS_REL_PATH);
    fs::create_dir_all(&thumb_path)?;

    println!("Thumbnailing {total_images} images...");
    let results: Vec<Result<()>> = image_paths
        .par_iter()
        .map(|img_path| thumbnailing_image(img_path, &thumb_path))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();
    let done_count = total_images.saturating_sub(err_count);
    println!("{done_count}/{total_images} done");
    results
        .iter()
        .filter_map(|result| result.as_ref().err())
        .take(10)
        .for_each(|err| eprintln!("\n{err:#}"));

    Ok(())
}
