use anyhow::{Result, bail};

use iie::hash::hash_image;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(img_path) = args.get(1) else {
        bail!("require image path");
    };

    let hash_string = hash_image(&image::open(img_path)?);

    println!("{hash_string}");

    Ok(())
}
