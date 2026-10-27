use std::path::Path;

use anyhow::Result;
use image_indexing_engine::scan::scan_images;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let paths = scan_images(Path::new(&args[1]))?;

    println!("{paths:#?}");

    Ok(())
}
