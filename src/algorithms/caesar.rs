/// Encode un texte en utilisant le chiffrement César avec une clé de décalage.
pub fn encode(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                (((c as u8 - first + key) % 26) + first) as char
            } else {
                c
            }
        })
        .collect()
}

/// Decode un texte encodé avec le chiffrement César et une clé de décalage.
pub fn decode(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                (((c as u8 - first + 26 - key) % 26) + first) as char
            } else {
                c
            }
        })
        .collect()
}
