use std::{path::Path, time::Instant};

use anyhow::Result;
use image_indexing_engine::{
    db,
    index::indexing_folder,
    paths::{DB_REL_PATH, prepare_dot_folder},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);

    let start = Instant::now();

    prepare_dot_folder(root, false, true)?;
    let indexed_images = indexing_folder(root, true)?;
    db::write(&root.join(DB_REL_PATH), &indexed_images)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
