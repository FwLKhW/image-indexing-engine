use std::path::Path;

use anyhow::Result;
use index::index::indexing_image;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let img_path = Path::new(&args[1]);

    let root = img_path.parent().unwrap();
    let data = indexing_image(img_path, root, true, true)?;

    println!("{data:#?}");

    Ok(())
}
