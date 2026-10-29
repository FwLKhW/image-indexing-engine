use std::path::Path;

use anyhow::Result;
use iie::metadata;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_img_path = String::from("fixtures/imgs/train.webp");
    let img_path = args.get(1).unwrap_or(&default_img_path);
    let img_path = Path::new(img_path);

    metadata::write(img_path, metadata::MetadataWriteAction::Rating(5))?;
    metadata::write(
        img_path,
        metadata::MetadataWriteAction::Tagging("example".to_string()),
    )?;

    Ok(())
}
