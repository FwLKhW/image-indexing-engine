use std::path::Path;

pub fn extract_palette<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<[f32; 3]>> {
    let img = image::open(path)?.to_rgb8();
    let (width, height) = img.dimensions();

    let input =
        okmain::InputImage::from_bytes(width as u16, height as u16, img.as_raw().as_slice())?;

    let srgb_colors = okmain::colors(input);

    let oklab_palette = srgb_colors
        .into_iter()
        .map(|c| {
            let r = c.r as f32 / 255.0;
            let g = c.g as f32 / 255.0;
            let b = c.b as f32 / 255.0;

            let lab = oklab::srgb_f32_to_oklab(rgb::Rgb { r, g, b });
            [lab.l, lab.a, lab.b]
        })
        .collect();

    Ok(oklab_palette)
}
