use anyhow::Result;
use image::EncodableLayout;

use index::hash::hash_image_bytes;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::open(&args[1])?;
    let hash_result = hash_image_bytes(img.to_rgba16().as_bytes());

    println!("{hash_result}");

    Ok(())
}
