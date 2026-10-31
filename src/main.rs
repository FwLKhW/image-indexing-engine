use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use clap::{Parser, Subcommand};

use colored_text::Colorize;
use iie::{
    db, index::indexing_folder, metadata::{self, MetadataWriteAction}, paths::{DB_REL_PATH, DOT_FOLDER}, thumbnail::generate_thumbnail_for_folder,
};

#[derive(Parser, Debug)]
#[command(
    version,
    name = "iie",
    about = "An image indexing engine.",
    long_about = "\n
    ███ ███ ███████
     █   █  █
     █   █  █
     █   █  █████
     █   █  █
     █   █  █
    ███ ███ ███████

An image indexing engine."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Clean dot folder
    Clean {
        /// Target folder
        #[arg(required = true)]
        folder_path: String,
    },
    /// Generate thumbnails
    Thumb {
        /// Target folder
        #[arg(required = true)]
        folder_path: String,
    },
    /// Indexing
    Index {
        /// Target folder
        folder_path: String,

        /// Enable extended indexing (hash, colors)
        #[arg(short, long, default_value_t = false)]
        extended: bool,
    },
    /// Write metadata to image
    Write {
        /// Image path
        image_path: String,

        #[command(subcommand)]
        command: WriteCommands,
    },
}

#[derive(Debug, Subcommand)]
enum WriteCommands {
    /// Set rating
    Rating {
        /// Rating value to be written (0 to 5)
        value: u8,
    },

    /// Add or remove tag
    Tag {
        /// Tag to add or remove. If not exist, add. Otherwise, remove.
        tag: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let start = Instant::now();

    match &cli.command {
        Commands::Clean { folder_path } => {
            let dot_folder_path = PathBuf::from(folder_path).join(DOT_FOLDER);
            fs::remove_dir_all(dot_folder_path)?;
        }
        Commands::Thumb { folder_path } => {
            let folder_path = PathBuf::from(folder_path);
            generate_thumbnail_for_folder(&folder_path)?;
        }
        Commands::Index {
            folder_path,
            extended,
        } => {
            let folder_path = PathBuf::from(folder_path);
            let indexed_images = indexing_folder(&folder_path, *extended)?;
            db::write(&folder_path.join(DB_REL_PATH), &indexed_images)?;
        }
        Commands::Write {
            image_path,
            command,
        } => {
            let img_path = PathBuf::from(image_path);
            match command {
                WriteCommands::Rating { value } => {
                    metadata::write(&img_path, MetadataWriteAction::Rating(*value))?;
                }
                WriteCommands::Tag { tag } => {
                    metadata::write(&img_path, MetadataWriteAction::Tagging(tag.clone()))?;
                }
            }
        }
    }

    println!(
        "\nTake {} seconds\n",
        format!("{}", start.elapsed().as_secs_f32()).green()
    );

    Ok(())
}
