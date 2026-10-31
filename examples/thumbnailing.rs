use std::{path::Path, time::Instant};

use anyhow::Result;
use iie::thumbnail::generate_thumbnail_for_folder;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_folder_path = String::from("fixtures/imgs/");
    let folder_path = args.get(1).unwrap_or(&default_folder_path);
    let folder_path = Path::new(folder_path);

    let start = Instant::now();

    generate_thumbnail_for_folder(folder_path)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
