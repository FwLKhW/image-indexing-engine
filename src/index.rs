use std::{
    fs::{self, create_dir_all, remove_dir_all},
    os::windows::fs::MetadataExt,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Ok, Result};
use image::{DynamicImage, EncodableLayout};
use path_slash::PathExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use smol_str::SmolStr;

use crate::{
    color::extract_palette,
    hash::hash_image,
    paths::{DOT_FOLDER, THUMBNAILS_PATH},
    scan::scan_images,
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

pub fn index_image(img_path: &Path, img: &DynamicImage, root: &Path) -> Result<IndexedImage> {
    let rgb8_img = img.to_rgb8();
    let rgba16_img = img.to_rgba16();

    let metadata = fs::metadata(img_path)?;
    let format = img_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let size = metadata.file_size();
    let rel_path = img_path
        .strip_prefix(root)
        .with_context(|| format!("failed strip prefix {root:?} from {img_path:?}"))?;

    Ok(IndexedImage {
        hash: hash_image(rgba16_img.as_bytes(), format, size),
        path: rel_path.to_slash_lossy().into(),
        name: img_path
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

    println!("Found {} images.", total_images);

    let dot_folder_path = root.join(DOT_FOLDER);
    remove_dir_all(&dot_folder_path).with_context(|| {
        format!("failed remove {dot_folder_path:?}, maybe used by another program")
    })?;

    let thumb_path = root.join(THUMBNAILS_PATH);
    create_dir_all(&thumb_path)?;

    let indexed_images = image_paths
        .par_iter()
        .map(|p| index_and_thumbnail(p, root, &thumb_path));

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
