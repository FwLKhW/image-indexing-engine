use colored_text::Colorize;
use image_indexing_engine::color::extract_palette_from_path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let colors = extract_palette_from_path(&args[1]);

    if let Ok(colors) = colors {
        println!("Dominant colors in Oklab:");

        colors.iter().for_each(|c| {
            let rgb = oklab::oklab_to_srgb(oklab::Oklab {
                l: c[0],
                a: c[1],
                b: c[2],
            });
            println!("{}", format!("{c:?}").on_rgb(rgb.r, rgb.g, rgb.b));
        })
    }
}
