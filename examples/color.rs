use anyhow::{Result, bail};
use colored_text::Colorize;

use index::color::extract_colors;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(img_path) = args.get(1) else {
        bail!("require image path");
    };
    let colors = extract_colors(&image::open(img_path)?)?;

    println!("Dominant colors in Oklab:");

    for rgb in &colors {
        println!("{}", format!("{rgb:?}").on_rgb(rgb.r, rgb.g, rgb.b));
    }

    Ok(())
}
