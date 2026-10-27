use std::path::Path;

use image_indexing_engine::{paths::clean_dot_folder, thumbnail::thumbnailing_folder};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);

    clean_dot_folder(&root)?;
    thumbnailing_folder(&root)?;

    Ok(())
}
