use std::path::Path;

use anyhow::Result;
use csv::Writer;

use crate::{color::rgb2hex, index::IndexedImage};

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
        "tags",
        "hash",
        "color_1",
        "color_2",
        "color_3",
        "color_4",
    ])?;

    let default_color = String::new();

    for (id, img) in indexed_images.iter().enumerate() {
        let colors_hex: Vec<String> = img.colors.iter().map(rgb2hex).collect();

        wtr.write_record([
            &id.to_string(),
            &img.path,
            &img.name,
            &img.format,
            &img.size.to_string(),
            &img.width.to_string(),
            &img.height.to_string(),
            &img.created.to_string(),
            &img.modified.to_string(),
            &img.rating.to_string(),
            &img.tags.join("|"),
            &img.hash,
            colors_hex.first().unwrap_or(&default_color),
            colors_hex.get(1).unwrap_or(&default_color),
            colors_hex.get(2).unwrap_or(&default_color),
            colors_hex.get(3).unwrap_or(&default_color),
        ])?;
    }

    wtr.flush()?;

    Ok(())
}
