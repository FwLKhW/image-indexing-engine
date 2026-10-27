use image::EncodableLayout;

use image_indexing_engine::hash::hash_image;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::open(&args[1])?;
    let hash_result = hash_image(img.to_rgba16().as_bytes());

    println!("{hash_result}");

    Ok(())
}
