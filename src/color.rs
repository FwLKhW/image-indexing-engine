use anyhow::Result;
use image::{ImageBuffer, Rgb};

/// # Errors
///
/// The only case this will fail is width or height of the image bigger than 65536.
pub fn extract_palette(rgb8_img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<Vec<[f32; 3]>> {
    let (width, height) = rgb8_img.dimensions();

    let input = okmain::InputImage::from_bytes(
        u16::try_from(width)?,
        u16::try_from(height)?,
        rgb8_img.as_raw(),
    )?;

    let srgb_colors = okmain::colors(input);

    let oklab_palette = srgb_colors
        .into_iter()
        .map(|c| {
            let r = f32::from(c.r) / 255.0;
            let g = f32::from(c.g) / 255.0;
            let b = f32::from(c.b) / 255.0;

            let lab = oklab::srgb_f32_to_oklab(rgb::Rgb { r, g, b });
            [lab.l, lab.a, lab.b]
        })
        .collect();

    Ok(oklab_palette)
}
