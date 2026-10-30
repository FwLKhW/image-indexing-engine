use anyhow::{Result, bail};

use iie::hash::blake3_hash;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(img_path) = args.get(1) else {
        bail!("require image path");
    };

    let hash_string = blake3_hash(&image::open(img_path)?);

    println!("{hash_string}");

    Ok(())
}
