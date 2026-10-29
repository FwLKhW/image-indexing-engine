use std::{fs, path::PathBuf, process::exit, time::Instant};

use anyhow::{Result, bail};
use clap::Parser;

use colored_text::Colorize;
use index::{
    db,
    index::indexing_folder,
    paths::{DB_REL_PATH, prepare_dot_folder},
    thumbnail::thumbnailing_folder,
};

#[derive(Parser, Debug)]
#[command(version, about = "An image indexing engine.")]
struct Args {
    /// Target folder
    folder_path: String,

    /// Enable generate thumbnails
    #[arg(short, long, default_value_t = false)]
    thumbnail: bool,

    /// Enable indexing
    #[arg(short, long, default_value_t = false)]
    index: bool,

    /// Enable color palette extraction
    #[arg(short, long, default_value_t = false)]
    palette: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !fs::exists(&args.folder_path).unwrap_or_default() {
        bail!("\"{}\" is not exists or not accessible", args.folder_path);
    }

    if !args.thumbnail && !args.index {
        println!("Do nothing :)");
        exit(0);
    }

    let start = Instant::now();

    let root = PathBuf::from(&args.folder_path);
    if args.index {
        prepare_dot_folder(&root, false, true)?;
        let indexed_images = indexing_folder(&root, args.palette)?;
        db::write(&root.join(DB_REL_PATH), &indexed_images)?;
        println!(
            "\nTake {} seconds\n",
            format!("{}", start.elapsed().as_secs_f32()).green()
        );
    }
    if args.thumbnail {
        prepare_dot_folder(&root, true, false)?;
        thumbnailing_folder(&root)?;
        println!(
            "\nTake {} seconds\n",
            format!("{}", start.elapsed().as_secs_f32()).green()
        );
    }

    Ok(())
}
