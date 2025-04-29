use base64::engine::general_purpose::STANDARD;
use base64::Engine as _; // Trait nécessaire

/// Encode un texte en Base64.
pub fn encode(text: &str) -> String {
    STANDARD.encode(text)
}

/// Decode un texte en Base64.
/// Renvoie un String ou un message d'erreur.
pub fn decode(text: &str) -> String {
    match STANDARD.decode(text) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(decoded_text) => decoded_text,
            Err(_) => "Erreur : données non valides en UTF-8".to_string(),
        },
        Err(_) => "Erreur : texte Base64 invalide".to_string(),
    }
}
