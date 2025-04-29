pub fn caesar_decode(text: &str, key: u8) -> String {
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
