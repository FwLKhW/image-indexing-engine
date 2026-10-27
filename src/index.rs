use std::{
    fs::{self, create_dir_all, remove_dir_all},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use image::{DynamicImage, EncodableLayout};
use path_slash::PathExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    color::extract_palette,
    hash::hash_image,
    paths::{DOT_FOLDER, THUMBNAILS_PATH},
    scan::scan_images,
    thumbnail::generate_thumbnail,
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
    pub rating: u32,
    pub description: String,
    pub tags: Vec<String>,
    pub hash: String,
    pub palette: Vec<[f32; 3]>,
}

pub fn index_image(img_path: &Path, img: &DynamicImage, root: &Path) -> Result<IndexedImage> {
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
        hash: hash_image(rgba16_img.as_bytes()),
        palette: extract_palette(&rgb8_img).unwrap_or_default(),
    })
}

pub fn index_and_thumbnail(
    img_path: &Path,
    root: &Path,
    thumb_path: &Path,
) -> Result<IndexedImage> {
    let img =
        image::open(img_path).with_context(|| format!("can not open image at {:?}", img_path))?;

    let indexed_image = index_image(img_path, &img, root)
        .with_context(|| format!("failed index image {:?}", img_path))?;
    let thumbnail = generate_thumbnail(&img);

    let thumbnail_save_path = thumb_path.join(format!("{}.png", indexed_image.hash));
    thumbnail.save(&thumbnail_save_path).with_context(|| {
        format!(
            "failed save thumbnail for {:?} at {:?}",
            img_path, thumbnail_save_path
        )
    })?;

    Ok(indexed_image)
}

pub fn index_folder(root: &Path) -> Result<Vec<IndexedImage>> {
    let image_paths = scan_images(root)?;
    let total_images = image_paths.len();

    let dot_folder_path = root.join(DOT_FOLDER);
    if let std::result::Result::Ok(exist) = dot_folder_path.try_exists()
        && exist
    {
        remove_dir_all(&dot_folder_path).with_context(|| {
            format!("failed remove {dot_folder_path:?}, maybe used by another program")
        })?;
    }

    let thumb_path = root.join(THUMBNAILS_PATH);
    create_dir_all(&thumb_path)?;

    println!("Indexing {} images...", total_images);

    let results: Vec<Result<IndexedImage>> = image_paths
        .par_iter()
        .map(|img_path| index_and_thumbnail(img_path, root, &thumb_path))
        .collect();

    let err_count = results.iter().filter(|i| i.is_err()).count();

    println!(
        "{}",
        if err_count == 0 {
            "All images indexed".into()
        } else {
            format!("{} images failed", err_count)
        }
    );

    if err_count < 10 {
        results.iter().for_each(|i| {
            if i.is_err() {
                println!("\n{i:?}\n")
            }
        });
    }

    let success_indexed_images: Vec<IndexedImage> =
        results.into_iter().filter_map(|i| i.ok()).collect();

    Ok(success_indexed_images)
}
