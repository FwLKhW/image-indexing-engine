use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
    time::Instant,
};

use anyhow::Result;
use clap::Parser;

use image_indexing_engine::{db, index::index_folder, paths::DB_PATH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    folder_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !fs::exists(&args.folder_path).unwrap_or(false) {
        eprintln!("\"{}\" is not accessible or not exists.", args.folder_path);
        exit(1);
    }

    let start = Instant::now();

    let indexed_images = index_folder(Path::new(&args.folder_path))?;
    let db_path = PathBuf::from(&args.folder_path).join(DB_PATH);
    db::write(&db_path, &indexed_images).await?;

    let duration = start.elapsed();

    println!(
        "Indexed {} images in {} seconds",
        indexed_images.len(),
        duration.as_secs_f32()
    );

    Ok(())
}
