use std::{
    fs,
    os::windows::fs::MetadataExt,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Ok;
use image::EncodableLayout;
use smol_str::SmolStr;

use crate::{color::extract_palette, hash::hash_image};

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

pub fn index_image<P: AsRef<Path>>(path: P) -> anyhow::Result<IndexedImage> {
    let img = image::open(&path)?;
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
        size: size,
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
        hash: hash_image(rgba16_img.as_bytes(), &format, size),
        palette: extract_palette(&rgb8_img).unwrap_or_default(),
    })
}

pub fn index_folder<P: AsRef<Path>>(path: P) {
    
}