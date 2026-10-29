use std::path::Path;

use anyhow::Result;
use csv::Writer;

use crate::index::IndexedImage;

pub fn write(csv_path: &Path, indexed_images: &[IndexedImage]) -> Result<()> {
    println!("Writing index.csv...");

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
        let id = (index + 1).to_string();
        let size_str = img.size.to_string();
        let width_str = img.width.to_string();
        let height_str = img.height.to_string();
        let created_str = img.created.to_string();
        let modified_str = img.modified.to_string();
        let rating_str = img.rating.to_string();

        let tags_json = serde_json::to_string(&img.tags).expect("Failed to serialize tags");
        let palette_json =
            serde_json::to_string(&img.palette).expect("Failed to serialize palette");

        wtr.write_record([
            &id,
            &img.path.to_string(),
            &img.name.to_string(),
            &img.format.to_string(),
            &size_str,
            &width_str,
            &height_str,
            &created_str,
            &modified_str,
            &rating_str,
            &img.description.to_string(),
            &tags_json,
            &img.hash,
            &palette_json,
        ])?;
    }

    wtr.flush()?;

    println!("{} records written", indexed_images.len());

    Ok(())
}
