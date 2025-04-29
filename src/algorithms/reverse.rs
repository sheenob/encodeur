/// Encode un texte en le renversant.
pub fn encode(text: &str) -> String {
    text.chars().rev().collect()
}

/// Decode un texte en le renversant à nouveau.
pub fn decode(text: &str) -> String {
    encode(text) // Reverse est symétrique
}
