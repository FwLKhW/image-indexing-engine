use std::path;

use duckdb::arrow::datatypes::ToByteSlice;
use filesize::PathExt;

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
    let size = path.size_on_disk()?;

    let hash_result = hash_image(&img.to_rgba16().to_byte_slice(), format, size);

    println!("{hash_result}");

    Ok(())
}
