use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Ok, Result};
use path_slash::PathExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rgb::Rgb;

use crate::{
    color::extract_colors, hash::blake3_hash, metadata, paths::DOT_FOLDER, scan::scan_images,
};

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
    pub tags: Vec<String>,
    pub hash: String,
    pub colors: Vec<Rgb<u8>>,
}

/// # Errors
///
/// Path does not exist.
/// Image path isn't a child of root.
/// Or failed reading system metadata and xmp metadata.
pub fn indexing_image(img_path: &Path, root: &Path, extended: bool) -> Result<IndexedImage> {
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

    let sys_meta = fs::metadata(img_path)?;
    let size = sys_meta.len();
    let created = u64::try_from(
        sys_meta
            .created()
            .unwrap_or_else(|_| SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )?;
    let modified = u64::try_from(
        sys_meta
            .modified()
            .unwrap_or_else(|_| SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )?;

    let dimensions = imagesize::size(img_path)?;
    let ext_meta = metadata::read(img_path)?;

    let mut indexed_image = IndexedImage {
        path: rel_path,
        name,
        format,
        size,
        width: u32::try_from(dimensions.width)?,
        height: u32::try_from(dimensions.height)?,
        created,
        modified,
        rating: ext_meta.rating,
        tags: ext_meta.tags,
        hash: String::new(),
        colors: vec![],
    };

    if extended {
        let img = image::open(img_path)?;
        indexed_image.hash = blake3_hash(&img);
        indexed_image.colors = extract_colors(&img)?;
    }

    Ok(indexed_image)
}

/// # Errors
///
/// Can not create dot folder or image indexing failed.
pub fn indexing_folder(root: &Path, extended: bool) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(root);
    let total_images = image_paths.len();

    let dot_folder_path = root.join(DOT_FOLDER);
    fs::create_dir_all(&dot_folder_path)?;

    println!("Indexing {total_images} images...");
    let results: Vec<Result<IndexedImage>> = image_paths
        .par_iter()
        .map(|img_path| indexing_image(img_path, root, extended))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();

    println!(
        "{}/{} done",
        total_images.saturating_sub(err_count),
        total_images
    );
    results
        .iter()
        .filter_map(|result| result.as_ref().err())
        .take(10)
        .for_each(|err| eprintln!("\n{err:#}"));

    let success_indexed_images: Vec<IndexedImage> =
        results.into_iter().filter_map(Result::ok).collect();

    Ok(success_indexed_images)
}
