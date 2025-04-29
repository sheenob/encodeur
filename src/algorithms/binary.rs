/// Encode un texte en binaire (ASCII -> bits).
pub fn encode(text: &str) -> String {
    text.bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Decode une chaîne binaire vers texte.
/// (Chaque octet est séparé par des espaces)
pub fn decode(text: &str) -> String {
    text.split_whitespace()
        .filter_map(|b| u8::from_str_radix(b, 2).ok())
        .map(|byte| byte as char)
        .collect()
}
