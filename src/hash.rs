pub fn hash_image(pixels: &[u8], format: &str) -> String {
    let mut hasher = blake3::Hasher::new();

    hasher.update(pixels);
    hasher.update(format.as_bytes());

    hasher.finalize().to_hex().to_string()
}
