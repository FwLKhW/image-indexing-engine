use image::{DynamicImage, EncodableLayout};

#[must_use]
pub fn hash_image_bytes(rgba16: &[u8]) -> String {
    blake3::hash(rgba16).to_hex().to_string()
}

#[must_use]
pub fn hash_image(img: &DynamicImage) -> String {
    hash_image_bytes(img.to_rgba16().as_bytes())
}
