use image::{DynamicImage, EncodableLayout};

#[must_use]
pub fn hash_image(img: &DynamicImage) -> String {
    blake3::hash(img.to_rgba16().as_bytes())
        .to_hex()
        .to_string()
}
