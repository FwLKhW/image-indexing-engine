use std::{fs, path::PathBuf, process::exit, time::Instant};

use anyhow::{Result, bail};
use clap::Parser;

use colored_text::Colorize;
use index::{
    csv,
    index::indexing_folder,
    paths::{CSV_REL_PATH, prepare_dot_folder},
    thumbnail::thumbnailing_folder,
};

#[derive(Parser, Debug)]
#[command(version, about = "An image indexing engine.")]
#[allow(clippy::struct_excessive_bools)]
struct Args {
    /// Target folder
    folder_path: String,

    /// Enable generate thumbnails
    #[arg(short = 't', long, default_value_t = false)]
    thumbnail: bool,

    /// Enable indexing
    #[arg(short = 'i', long, default_value_t = false)]
    index: bool,

    /// Enable calculate pixels hash for duplicate detection
    #[arg(short = 's', long, default_value_t = false)]
    hash: bool,

    /// Enable color palette extraction (dynamic 1 to 5 colors)
    #[arg(short = 'p', long, default_value_t = false)]
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
        let indexed_images = indexing_folder(&root, args.hash, args.palette)?;
        csv::write(&root.join(CSV_REL_PATH), &indexed_images)?;
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
