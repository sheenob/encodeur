/// Encode un texte en appliquant un XOR avec une clé simple.
pub fn encode(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}

/// Decode un texte encodé en XOR.
/// (XOR est symétrique, encode et decode sont identiques)
pub fn decode(text: &str, key: u8) -> String {
    encode(text, key)
}
