use image_indexing_engine::index::index_folder;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let indexed_images = index_folder(&args[1])?;

    println!("{indexed_images:#?}");

    Ok(())
}
