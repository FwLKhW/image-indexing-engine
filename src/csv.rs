use std::path::Path;

use anyhow::{Context, Result};
use csv::Writer;

use crate::index::IndexedImage;

/// # Errors
///
/// `IndexedImage` contain `tags` or `palette` isn't a JSON string.
pub fn write(csv_path: &Path, indexed_images: &[IndexedImage]) -> Result<()> {
    let mut wtr = Writer::from_path(csv_path)?;

    wtr.write_record([
        "id",
        "path",
        "name",
        "format",
        "size",
        "width",
        "height",
        "created",
        "modified",
        "rating",
        "description",
        "tags",
        "hash",
        "palette",
    ])?;

    for (index, img) in indexed_images.iter().enumerate() {
        let id = index.to_string();
        let size_str = img.size.to_string();
        let width_str = img.width.to_string();
        let height_str = img.height.to_string();
        let created_str = img.created.to_string();
        let modified_str = img.modified.to_string();
        let rating_str = img.rating.to_string();

        let tags_json = serde_json::to_string(&img.tags).context("Failed to serialize tags")?;
        let palette_json =
            serde_json::to_string(&img.palette).context("Failed to serialize palette")?;

        wtr.write_record([
            &id,
            &img.path,
            &img.name,
            &img.format,
            &size_str,
            &width_str,
            &height_str,
            &created_str,
            &modified_str,
            &rating_str,
            &img.description,
            &tags_json,
            &img.hash,
            &palette_json,
        ])?;
    }

    wtr.flush()?;

    Ok(())
}
