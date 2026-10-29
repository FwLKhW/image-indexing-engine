use std::path::Path;

use anyhow::{Result, bail};
use index::scan::scan_images;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(path) = args.get(1) else {
        bail!("require file path");
    };

    let paths = scan_images(Path::new(path));

    println!("{paths:#?}");

    Ok(())
}
