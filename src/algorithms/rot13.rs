/// Encode (et décode) un texte en utilisant ROT13.
/// (encode et decode sont identiques pour ROT13)
pub fn encode(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                (((c as u8 - base + 13) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

/// Decode un texte encodé avec ROT13.
/// (identique à encode pour ROT13)
pub fn decode(text: &str) -> String {
    encode(text) // ROT13 est symétrique
}
