use std::{path::Path, time::Instant};

use anyhow::Result;
use image_indexing_engine::{paths::prepare_dot_folder, thumbnail::thumbnailing_folder};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);

    let start = Instant::now();

    prepare_dot_folder(root, true, false)?;
    thumbnailing_folder(&root)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
