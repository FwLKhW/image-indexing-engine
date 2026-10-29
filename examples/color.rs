use anyhow::{Result, bail};
use colored_text::Colorize;

use index::color::extract_palette;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let Some(img_path) = args.get(1) else {
        bail!("require image path");
    };
    let oklab_colors = extract_palette(&image::open(img_path)?.to_rgb8())?;

    println!("Dominant colors in Oklab:");

    for c in &oklab_colors {
        let rgb = oklab::oklab_to_srgb(oklab::Oklab {
            l: c[0],
            a: c[1],
            b: c[2],
        });
        println!("{}", format!("{c:?}").on_rgb(rgb.r, rgb.g, rgb.b));
    }

    Ok(())
}
