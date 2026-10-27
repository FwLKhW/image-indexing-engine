use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use image::EncodableLayout;
use path_slash::PathExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{color::extract_palette, hash::hash_image_bytes, scan::scan_images};

#[derive(Debug)]
pub struct IndexedImage {
    pub path: String,
    pub name: String,
    pub format: String,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub created: u64,
    pub modified: u64,
    pub rating: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub hash: String,
    pub palette: Vec<[f32; 3]>,
}

pub fn indexing_image(img_path: &Path, root: &Path) -> Result<IndexedImage> {
    let img = image::open(img_path).with_context(|| format!("failed open image {:?}", img_path))?;
    let rgb8_img = img.to_rgb8();
    let rgba16_img = img.to_rgba16();

    let metadata = fs::metadata(img_path)?;

    let rel_path = img_path.strip_prefix(root)?.to_slash_lossy().to_string();
    let name = img_path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();
    let format = img_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_uppercase();
    let size = metadata.len();
    let created = metadata
        .created()
        .unwrap_or(SystemTime::now())
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let modified = metadata
        .modified()
        .unwrap_or(SystemTime::now())
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    Ok(IndexedImage {
        path: rel_path,
        name,
        format,
        size,
        width: img.width(),
        height: img.height(),
        created,
        modified,
        rating: 0,
        description: "".into(),
        tags: vec![],
        hash: hash_image_bytes(rgba16_img.as_bytes()),
        palette: extract_palette(&rgb8_img).unwrap_or_default(),
    })
}

pub fn index_folder(root: &Path) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(root)?;
    let total_images = image_paths.len();

    println!("Indexing {} images...", total_images);

    let results: Vec<Result<IndexedImage>> = image_paths
        .par_iter()
        .map(|img_path| indexing_image(img_path, root))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();

    println!("{}/{} done", total_images - err_count, total_images);
    results.iter().take(10).for_each(|result| {
        if let Err(err) = result {
            eprintln!("\n{err:#}")
        }
    });

    let success_indexed_images: Vec<IndexedImage> =
        results.into_iter().filter_map(|i| i.ok()).collect();

    Ok(success_indexed_images)
}
