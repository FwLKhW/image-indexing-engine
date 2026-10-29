use std::{path::Path, time::Instant};

use anyhow::Result;
use index::{
    csv,
    index::indexing_folder,
    paths::{CSV_REL_PATH, prepare_dot_folder},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);

    let start = Instant::now();

    prepare_dot_folder(root, false, true)?;
    let indexed_images = indexing_folder(root, true, true)?;

    let db_path = root.join(CSV_REL_PATH);
    csv::write(&db_path, &indexed_images)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
