use std::{path::Path, process::exit};

use anyhow::Result;
use index::metadata::{self, Metadata};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = Path::new(&args[1]);
    let meta = metadata::read(path)?;

    println!("{meta:#?}");

    if args.len() <= 2 {
        exit(0);
    }

    if let Ok(rating) = &args[2].parse::<u8>() {
        println!("Set rating {}", rating);

        metadata::write(
            path,
            &Metadata {
                rating: *rating,
                ..Default::default()
            },
        )?;
    }

    let meta = metadata::read(path)?;
    println!("{meta:#?}");

    Ok(())
}
