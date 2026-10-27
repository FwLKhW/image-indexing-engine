use std::path::Path;

use image_indexing_engine::index::indexing_image;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let img_path = Path::new(&args[1]);

    let root = img_path.parent().unwrap();
    let data = indexing_image(img_path, root, true)?;

    println!("{data:#?}");

    Ok(())
}
