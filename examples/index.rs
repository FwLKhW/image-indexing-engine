use std::path::Path;

use image_indexing_engine::index::index_image;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img_path = Path::new(&args[1]);
    let img = image::open(&args[1])?;
    let root = img_path.parent().unwrap();

    let data = index_image(img_path, &img, root)?;

    println!("{data:#?}");

    Ok(())
}
