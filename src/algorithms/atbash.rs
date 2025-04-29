/// Encode (et decode) un texte en utilisant le chiffrement Atbash.
pub fn encode(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                // A -> Z, B -> Y, etc.
                (b'Z' - (c as u8 - b'A')) as char
            } else if c.is_ascii_lowercase() {
                (b'z' - (c as u8 - b'a')) as char
            } else {
                c
            }
        })
        .collect()
}

/// Decode un texte en utilisant Atbash.
/// (Identique à encode car Atbash est symétrique)
pub fn decode(text: &str) -> String {
    encode(text)
}
