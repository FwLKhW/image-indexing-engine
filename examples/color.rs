use anyhow::Result;
use colored_text::Colorize;

use index::color::extract_palette_from_path;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let colors = extract_palette_from_path(&args[1])?;

    println!("Dominant colors in Oklab:");

    colors.iter().for_each(|c| {
        let rgb = oklab::oklab_to_srgb(oklab::Oklab {
            l: c[0],
            a: c[1],
            b: c[2],
        });
        println!("{}", format!("{c:?}").on_rgb(rgb.r, rgb.g, rgb.b));
    });

    Ok(())
}
