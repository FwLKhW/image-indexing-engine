use std::{fs, path::PathBuf, process::exit};

use anyhow::Result;
use clap::Parser;

use image_indexing_engine::{db, index::index_folder, paths::DB_PATH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    folder_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !fs::exists(&args.folder_path).unwrap_or(false) {
        eprintln!("\"{}\" is not accessible or not exists.", args.folder_path);
        exit(1);
    }

    let indexed_images = index_folder(&args.folder_path)?;

    let db_path = PathBuf::from(&args.folder_path).join(DB_PATH);

    db::write(db_path, indexed_images)?;

    Ok(())
}
