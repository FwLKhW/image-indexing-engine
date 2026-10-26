use image_indexing_engine::index::index_image;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::open(&args[1])?;
    let data = index_image(&args[1], &img)?;

    println!("{data:#?}");

    Ok(())
}
