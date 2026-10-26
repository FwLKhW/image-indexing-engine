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
    color::extract_palette, hash::hash_image, paths::THUMBNAILS_PATH, scan::scan_images,
    thumbnail::generate_thumbnail,
};

#[derive(Debug)]
pub struct IndexedImage {
    pub hash: String,
    pub path: SmolStr,
    pub name: SmolStr,
    pub format: SmolStr,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub created: u64,
    pub modified: u64,
    pub rating: u32,
    pub description: SmolStr,
    pub tags: Vec<String>,
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
        hash: hash_image(rgba16_img.as_bytes(), format, size),
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
        rating: 0,
        description: "".into(),
        tags: vec![],
        palette: extract_palette(&rgb8_img).unwrap_or_default(),
    })
}

pub fn index_and_thumbnail<P: AsRef<Path>>(image_path: P, thumb_path: P) -> Result<IndexedImage> {
    let img = image::open(&image_path)
        .with_context(|| format!("can not open image at {:?}", image_path.as_ref()))?;

    let indexed_image = index_image(&image_path, &img)
        .with_context(|| format!("failed index image {:?}", image_path.as_ref()))?;
    let thumbnail = generate_thumbnail(&img);

    let thumbnail_save_path = thumb_path
        .as_ref()
        .join(format!("{}.png", indexed_image.hash));
    thumbnail.save(&thumbnail_save_path).with_context(|| {
        format!(
            "failed save thumbnail for {:?} at {:?}",
            image_path.as_ref(),
            thumbnail_save_path
        )
    })?;

    Ok(indexed_image)
}

pub fn index_folder<P: AsRef<Path>>(path: P) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(&path)?;
    let total_images = image_paths.len();

    println!("Found {} images.", total_images);

    let thumb_path = path.as_ref().join(THUMBNAILS_PATH);
    create_dir_all(&thumb_path)?;

    let indexed_images = image_paths
        .par_iter()
        .map(|p| index_and_thumbnail(p, &thumb_path));

    let err_count = indexed_images.clone().filter(|i| i.is_err()).count();

    println!(
        "Indexed {}/{} images.",
        total_images - err_count,
        total_images
    );

    if err_count < 100 {
        indexed_images.clone().for_each(|i| {
            if i.is_err() {
                println!("{i:?}")
            }
        });
    }

    let success_indexed_images: Vec<IndexedImage> = indexed_images.filter_map(|i| i.ok()).collect();

    Ok(success_indexed_images)
}
