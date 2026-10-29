use anyhow::Result;
use colored_text::Colorize;

use iie::color::{extract_colors, rgb2hex};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_img_path = String::from("fixtures/imgs/train.webp");
    let img_path = args.get(1).unwrap_or(&default_img_path);

    let colors = extract_colors(&image::open(img_path)?)?;

    println!("Dominant colors of {img_path}:");

    for rgb in &colors {
        println!(
            "{}",
            format!("{} {rgb:?}", rgb2hex(rgb)).on_rgb(rgb.r, rgb.g, rgb.b)
        );
    }

    Ok(())
}
