use std::{fs, path::Path};

use anyhow::{Context, Result};

pub const DOT_FOLDER: &str = ".eyeris";

pub const THUMBNAILS_PATH: &str = ".eyeris/thumbnails/";
pub const DB_PATH: &str = ".eyeris/images.db";

pub fn clean_dot_folder(root: &Path) -> Result<()> {
    let dot_folder_path = root.join(DOT_FOLDER);

    if let std::result::Result::Ok(exist) = dot_folder_path.try_exists()
        && exist
    {
        fs::remove_dir_all(&dot_folder_path).with_context(|| {
            format!("failed remove {dot_folder_path:?}, maybe used by another program")
        })?;
    }

    let thumb_path = root.join(THUMBNAILS_PATH);
    fs::create_dir_all(&thumb_path)?;

    Ok(())
}
