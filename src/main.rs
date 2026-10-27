use std::{fs, path::PathBuf, time::Instant};

use anyhow::{Result, bail};
use clap::Parser;

use image_indexing_engine::{
    db, index::indexing_folder, paths::DB_REL_PATH, thumbnail::thumbnailing_folder,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Target folder
    folder_path: String,

    /// Enable generate thumbnails
    #[arg(short, long, default_value_t = false)]
    thumbnail: bool,

    /// Enable indexing
    #[arg(short, long, default_value_t = false)]
    index: bool,

    /// Enable calculate pixels hash (indexing)
    #[arg(long, default_value_t = false)]
    hash: bool,

    /// Enable dominant colors extraction (indexing)
    #[arg(short, long, default_value_t = false)]
    color: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !fs::exists(&args.folder_path).unwrap_or_default() {
        bail!("\"{}\" is not exists or not accessible", args.folder_path);
    }

    let start = Instant::now();

    let root = PathBuf::from(&args.folder_path);
    if args.index {
        let indexed_images = indexing_folder(&root)?;
        db::write(&root.join(DB_REL_PATH), &indexed_images).await?;
    }
    if args.thumbnail {
        thumbnailing_folder(&root)?;
    }

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
