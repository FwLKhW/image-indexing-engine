use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn scan_images(path: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();
    let mut stack = vec![path.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        let entries = match fs::read_dir(current_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let file_name = entry.file_name();
            let Some(name_str) = file_name.to_str() else {
                continue;
            };

            if name_str.starts_with('.') {
                continue;
            }

            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            if file_type.is_dir() {
                stack.push(entry.path());
            } else if file_type.is_file() {
                if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_ascii_lowercase();
                    // TODO: fixed file exts seem not right
                    if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "webp") {
                        images.push(entry.path());
                    }
                }
            }
        }
    }

    Ok(images)
}
