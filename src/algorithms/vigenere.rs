/// Encode un texte avec le chiffrement de Vigenère.
pub fn encode(text: &str, keyword: &str) -> String {
    let keyword = keyword.to_lowercase();
    let keyword_bytes = keyword.as_bytes();
    let mut result = String::new();

    let mut key_index = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let key = (keyword_bytes[key_index % keyword_bytes.len()] - b'a') % 26;
            let encoded_char = (((c as u8 - base + key) % 26) + base) as char;
            result.push(encoded_char);
            key_index += 1;
        } else {
            result.push(c);
        }
    }

    result
}

/// Decode un texte chiffré avec Vigenère.
pub fn decode(text: &str, keyword: &str) -> String {
    let keyword = keyword.to_lowercase();
    let keyword_bytes = keyword.as_bytes();
    let mut result = String::new();

    let mut key_index = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let key = (keyword_bytes[key_index % keyword_bytes.len()] - b'a') % 26;
            let decoded_char = (((c as u8 - base + 26 - key) % 26) + base) as char;
            result.push(decoded_char);
            key_index += 1;
        } else {
            result.push(c);
        }
    }

    result
}
