use std::{path::Path, time::Instant};

use anyhow::{Result, bail};
use iie::{paths::prepare_dot_folder, thumbnail::thumbnailing_folder};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(path) = args.get(1) else {
        bail!("require folder path");
    };

    let start = Instant::now();

    let root = Path::new(path);
    prepare_dot_folder(root, true, false)?;
    thumbnailing_folder(root)?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
