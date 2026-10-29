use std::path::Path;

use anyhow::{Result, bail};
use index::metadata;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(path) = args.get(1) else {
        bail!("require file path");
    };

    let path = Path::new(path);
    let meta = metadata::read(path)?;

    println!("{meta:#?}");

    Ok(())
}
