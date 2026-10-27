use std::path::{Path, PathBuf};

use anyhow::Result;
use dirwalk::WalkBuilder;

pub fn scan_images(path: &Path) -> Result<Vec<PathBuf>> {
    let result = WalkBuilder::new(path)
        .extensions(["jpg", "jpeg", "png", "webp", "gif", "avif", "bmp", "ico"])
        .dirs_first(true)
        .build()
        .unwrap();

    Ok(result
        .entries
        .iter()
        .filter(|e| !e.is_dir)
        .map(|e| path.join(&e.relative_path))
        .collect())
}
