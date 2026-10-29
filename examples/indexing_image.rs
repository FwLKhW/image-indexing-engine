use std::path::Path;

use anyhow::{Result, bail};
use index::index::indexing_image;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(img_path) = args.get(1) else {
        bail!("require image path");
    };
    let img_path = Path::new(img_path);

    let Some(root) = img_path.parent() else {
        bail!("failed get image parent folder");
    };
    let data = indexing_image(img_path, root, true, true)?;

    println!("{data:#?}");

    Ok(())
}
