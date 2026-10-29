use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use image::EncodableLayout;
use path_slash::PathExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{color::extract_palette, hash::hash_image_bytes, metadata, scan::scan_images};

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
    pub rating: u8,
    pub description: String,
    pub tags: Vec<String>,
    pub hash: String,
    pub palette: Vec<[f32; 3]>,
}

pub fn indexing_image(
    img_path: &Path,
    root: &Path,
    hash: bool,
    palette: bool,
) -> Result<IndexedImage> {
    let img = image::open(img_path).with_context(|| format!("failed open image {:?}", img_path))?;

    let sys_meta = fs::metadata(img_path)?;
    let ext_meta = metadata::read(img_path)?;

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
    let size = sys_meta.len();
    let created = sys_meta
        .created()
        .unwrap_or(SystemTime::now())
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let modified = sys_meta
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
        rating: ext_meta.rating,
        description: ext_meta.description,
        tags: ext_meta.tags,
        hash: if hash {
            hash_image_bytes(img.to_rgba16().as_bytes())
        } else {
            "".to_string()
        },
        palette: if palette {
            extract_palette(&img.to_rgb8()).unwrap_or_default()
        } else {
            vec![]
        },
    })
}

pub fn indexing_folder(root: &Path, hash: bool, palette: bool) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(root)?;
    let total_images = image_paths.len();

    println!("Indexing {} images...", total_images);

    let results: Vec<Result<IndexedImage>> = image_paths
        .par_iter()
        .map(|img_path| indexing_image(img_path, root, hash, palette))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();

    println!("{}/{} done", total_images - err_count, total_images);
    results
        .iter()
        .filter_map(|result| result.as_ref().err())
        .take(10)
        .for_each(|err| eprintln!("\n{err:#}"));

    let success_indexed_images: Vec<IndexedImage> =
        results.into_iter().filter_map(|i| i.ok()).collect();

    Ok(success_indexed_images)
}
