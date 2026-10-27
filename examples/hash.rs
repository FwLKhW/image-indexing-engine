use std::path;

use image::EncodableLayout;

use image_indexing_engine::hash::hash_image;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::open(&args[1])?;
    let path = path::PathBuf::from(&args[1]);
    let format = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let hash_result = hash_image(img.to_rgba16().as_bytes(), format);

    println!("{hash_result}");

    Ok(())
}
