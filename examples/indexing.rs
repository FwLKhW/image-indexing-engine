use image_indexing_engine::indexing::indexing;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let data = indexing(&args[1])?;

    println!("{data:#?}");

    Ok(())
}
