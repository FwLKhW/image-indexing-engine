use std::path::{self, Path, PathBuf};

use anyhow::Context;
use dirwalk::WalkBuilder;

pub fn scan_images<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
    let result = WalkBuilder::new(&path)
        .extensions(&["jpg", "jpeg", "png", "webp", "gif", "avif", "bmp", "ico"])
        .dirs_first(true)
        .build()
        .unwrap();

    let abs_path = path::absolute(&path).context("path can not parsed to absolute")?;

    Ok(result
        .entries
        .iter()
        .filter(|e| !e.is_dir)
        .map(|e| abs_path.join(&e.relative_path))
        .collect())
}
