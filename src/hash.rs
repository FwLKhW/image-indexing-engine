pub fn hash_image(pixels: &[u8], format: &str, size: u64) -> String {
    let mut hasher = blake3::Hasher::new();

    hasher.update(pixels);
    hasher.update(format.as_bytes());
    hasher.update(&size.to_be_bytes());

    hasher.finalize().to_hex().to_string()
}
