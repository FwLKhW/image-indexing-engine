use std::{
    fs::{self, create_dir_all},
    os::windows::fs::MetadataExt,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use image::{DynamicImage, EncodableLayout};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use smol_str::SmolStr;

use crate::{
    color::extract_palette, hash::hash_image, scan::scan_images, thumbnail::generate_thumbnail,
};

const DOT_FOLDER: &str = ".eyeris";

#[derive(Debug)]
pub struct IndexedImage {
    pub path: SmolStr,
    pub name: SmolStr,
    pub format: SmolStr,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub created: u64,
    pub modified: u64,
    pub hash: String,
    pub palette: Vec<[f32; 3]>,
}

pub fn index_image<P: AsRef<Path>>(path: P, img: &DynamicImage) -> Result<IndexedImage> {
    let rgb8_img = img.to_rgb8();
    let rgba16_img = img.to_rgba16();

    let metadata = fs::metadata(&path)?;
    let format = path
        .as_ref()
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let size = metadata.file_size();

    Ok(IndexedImage {
        path: path.as_ref().to_string_lossy().into(),
        name: path
            .as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .into(),
        format: format.into(),
        size,
        width: img.width(),
        height: img.height(),
        created: metadata
            .created()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        modified: metadata
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        hash: hash_image(rgba16_img.as_bytes(), format, size),
        palette: extract_palette(&rgb8_img).unwrap_or_default(),
    })
}

pub fn index_and_thumbnail<P: AsRef<Path>>(image_path: P, thumb_path: P) -> Result<IndexedImage> {
    let img = image::open(&image_path).context("can not open image path")?;

    let indexed_image = index_image(&image_path, &img).context("failed index image")?;
    let thumbnail = generate_thumbnail(&img);

    let file_name = &indexed_image.hash;
    thumbnail
        .save(thumb_path.as_ref().join(format!("{file_name}.png")))
        .context("failed save thumbnail")?;

    Ok(indexed_image)
}

pub fn index_folder<P: AsRef<Path>>(path: P) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(&path)?;

    println!("Found {} image.", image_paths.len());

    let thumb_path = path.as_ref().join(format!("{DOT_FOLDER}/thumbnails"));
    create_dir_all(&thumb_path)?;

    let indexed_images: Vec<IndexedImage> = image_paths
        .par_iter()
        .filter_map(|p| index_and_thumbnail(p, &thumb_path).ok())
        .collect();

    println!(
        "Indexed {}/{} images.",
        indexed_images.len(),
        image_paths.len()
    );

    Ok(indexed_images)
}
