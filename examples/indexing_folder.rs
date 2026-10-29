use std::{path::Path, time::Instant};

use anyhow::Result;
use iie::{csv, index::indexing_folder, paths::CSV_REL_PATH};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_folder_path = String::from("fixtures/imgs/");
    let folder_path = args.get(1).unwrap_or(&default_folder_path);
    let folder_path = Path::new(folder_path);

    let start = Instant::now();

    let indexed_images = indexing_folder(folder_path, true)?;
    let csv_path = folder_path.join(CSV_REL_PATH);
    csv::write(&csv_path, &indexed_images)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
