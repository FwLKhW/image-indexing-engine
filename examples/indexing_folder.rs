use std::{path::Path, time::Instant};

use anyhow::{Result, bail};
use index::{
    csv,
    index::indexing_folder,
    paths::{CSV_REL_PATH, prepare_dot_folder},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(path) = args.get(1) else {
        bail!("require folder path");
    };

    let start = Instant::now();

    let root = Path::new(path);
    prepare_dot_folder(root, false, true)?;
    let indexed_images = indexing_folder(root, true, true)?;

    let db_path = root.join(CSV_REL_PATH);
    csv::write(&db_path, &indexed_images)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
