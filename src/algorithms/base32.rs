use base32::{Alphabet, decode as base32_decode_lib, encode as base32_encode_lib};

/// Encode un texte en Base32.
pub fn encode(text: &str) -> String {
    base32_encode_lib(Alphabet::RFC4648 { padding: true }, text.as_bytes())
}

/// Decode un texte Base32.
pub fn decode(text: &str) -> String {
    match base32_decode_lib(Alphabet::RFC4648 { padding: true }, text) {
        Some(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => "Erreur : données Base32 invalides.".to_string(),
        },
        _none => "Erreur : texte Base32 invalide.".to_string(),
    }
}
