use std::{fs, path::Path};

use anyhow::{Context, Result, bail};

pub const DOT_FOLDER: &str = ".eyeris";

pub const THUMBNAILS_REL_PATH: &str = ".eyeris/thumbnails/";
pub const CSV_REL_PATH: &str = ".eyeris/index.csv";

pub fn prepare_dot_folder(
    root: &Path,
    delete_thumbnails: bool,
    delete_database: bool,
) -> Result<()> {
    if !fs::exists(root).unwrap_or(false) {
        bail!("\"{:?}\" is not exists or not accessible.", root)
    }

    let dot_folder_path = root.join(DOT_FOLDER);
    let thumbnails_path = root.join(THUMBNAILS_REL_PATH);
    let db_path = root.join(CSV_REL_PATH);

    if delete_thumbnails && thumbnails_path.try_exists().unwrap_or_default() {
        fs::remove_dir_all(&thumbnails_path).context("failed remove thumbnails folder")?;
    }
    if delete_database && db_path.try_exists().unwrap_or_default() {
        fs::remove_file(db_path).context("failed remove database file")?;
    }

    fs::create_dir_all(dot_folder_path).context("failed create dot folder")?;
    fs::create_dir_all(thumbnails_path)?;

    Ok(())
}
