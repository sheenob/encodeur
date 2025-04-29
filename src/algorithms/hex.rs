/// Encode un texte en Hexadécimal.
pub fn encode(text: &str) -> String {
    hex::encode(text)
}

/// Decode un texte en Hexadécimal.
/// Renvoie un String ou un message d'erreur.
pub fn decode(text: &str) -> String {
    match hex::decode(text) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(decoded_text) => decoded_text,
            Err(_) => "Erreur : données non valides en UTF-8".to_string(),
        },
        Err(_) => "Erreur : texte Hex invalide".to_string(),
    }
}
