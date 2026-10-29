use std::path::Path;

use anyhow::{Result, bail};
use iie::index::indexing_image;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_img_path = String::from("fixtures/imgs/KF51.webp");
    let img_path = args.get(1).unwrap_or(&default_img_path);
    let img_path = Path::new(img_path);

    let Some(root) = img_path.parent() else {
        bail!("failed get image parent folder");
    };
    let data = indexing_image(img_path, root, true)?;

    println!("{data:#?}");

    Ok(())
}
