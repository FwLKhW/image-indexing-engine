pub fn hash_image(pixels: &[u8]) -> String {
    blake3::hash(pixels).to_hex().to_string()
}
