use image_indexing_engine::thumbnail::generate_512px_thumbnail;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let thumbnail = generate_512px_thumbnail(&args[1]);

    if let Ok(thumbnail) = thumbnail {
        println!("Save thumbnail");
        thumbnail.save("fixtures/thumbnail.png")?;
    }

    Ok(())
}
