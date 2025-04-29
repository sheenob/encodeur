use std::fs;
use std::io::{self, Write};

mod algorithms;
mod types;
use algorithms::*;
use types::{Algo, Mode, SourceType};

fn main() {
    println!("===== Encodeur/Décodeur Rust =====\n");

    let mode = ask_mode();
    let algo = ask_algorithm();
    let source_type = ask_source_type();
    let input_text = match source_type {
        SourceType::Manual => ask_text(),
        SourceType::File => {
            let file_path = ask_file_path();
            match fs::read_to_string(&file_path) {
                Ok(content) => content.trim().to_string(),
                Err(_) => {
                    println!("Erreur lors de la lecture du fichier.");
                    return;
                }
            }
        }
    };

    let key = if matches!(algo, Algo::Cesar | Algo::Xor) {
        ask_key()
    } else {
        3 // clé par défaut pour les autres algos
    };

    let result = match (mode, algo) {
        (Mode::Encode, Algo::Cesar) => caesar_encode(&input_text, key),
        (Mode::Decode, Algo::Cesar) => caesar_decode(&input_text, key),
        (Mode::Encode, Algo::Rot13) => rot13_encode(&input_text),
        (Mode::Decode, Algo::Rot13) => rot13_decode(&input_text),
        (Mode::Encode, Algo::Base64) => base64_encode(&input_text),
        (Mode::Decode, Algo::Base64) => base64_decode(&input_text),
        (Mode::Encode, Algo::Hex) => hex_encode(&input_text),
        (Mode::Decode, Algo::Hex) => hex_decode(&input_text),
        (Mode::Encode, Algo::Atbash) => atbash_encode(&input_text),
        (Mode::Decode, Algo::Atbash) => atbash_decode(&input_text),
        (Mode::Encode, Algo::Xor) => xor_encode(&input_text, key),
        (Mode::Decode, Algo::Xor) => xor_decode(&input_text, key),
        (Mode::Encode, Algo::Reverse) => reverse_encode(&input_text),
        (Mode::Decode, Algo::Reverse) => reverse_decode(&input_text),
        (Mode::Encode, Algo::Binary) => binary_encode(&input_text),
        (Mode::Decode, Algo::Binary) => binary_decode(&input_text),
        (Mode::Encode, Algo::Base32) => base32_encode(&input_text),
        (Mode::Decode, Algo::Base32) => base32_decode(&input_text), 
    };

    println!("\nRésultat : {}\n", result);

    if let SourceType::File = source_type {
        let output_path = ask_output_file_path();
        if let Err(e) = fs::write(&output_path, result) {
            println!("Erreur lors de l'écriture du fichier : {}", e);
        } else {
            println!("Résultat sauvegardé dans {}", output_path);
        }
    }
}

// ------------------------------------------------------------
// FONCTIONS UTILES
// ------------------------------------------------------------



fn ask_mode() -> Mode {
    println!("1) Encoder\n2) Décoder");
    print!("Choisissez une option (1 ou 2) : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => Mode::Encode,
        "2" => Mode::Decode,
        _ => {
            println!("Entrée invalide, défaut sur Encode.");
            Mode::Encode
        }
    }
}

fn ask_algorithm() -> Algo {
    println!("\nListe des algorithmes disponibles :");
    println!("1) César\n2) ROT13\n3) Base64\n4) Hex\n5) Atbash\n6) XOR\n7) Reverse\n8) Binary\n9) Base32");
    print!("Choisissez votre algorithme : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => Algo::Cesar,
        "2" => Algo::Rot13,
        "3" => Algo::Base64,
        "4" => Algo::Hex,
        "5" => Algo::Atbash,
        "6" => Algo::Xor,
        "7" => Algo::Reverse,
        "8" => Algo::Binary,
        "9" => Algo::Base32,
        _ => {
            println!("Entrée invalide, défaut sur César.");
            Algo::Cesar
        }
    }
}

fn ask_source_type() -> SourceType {
    println!("\nSouhaitez-vous travailler avec :\n1) Texte manuel\n2) Fichier");
    print!("Votre choix (1 ou 2) : ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => SourceType::Manual,
        "2" => SourceType::File,
        _ => {
            println!("Entrée invalide, défaut sur Texte manuel.");
            SourceType::Manual
        }
    }
}

fn ask_text() -> String {
    print!("\nEntrez le texte : ");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    text.trim().to_string()
}

fn ask_file_path() -> String {
    loop {
        print!("\nEntrez le chemin du fichier à lire (doit être dans le dossier 'file_input/') : ");
        io::stdout().flush().unwrap();

        let mut path = String::new();
        io::stdin().read_line(&mut path).unwrap();
        let trimmed_path = path.trim();

        let full_path = format!("file_input/{}", trimmed_path);
        if std::path::Path::new(&full_path).exists() {
            return full_path;
        } else {
            println!("Erreur : le fichier '{}' n'existe pas dans le dossier 'file_input/'.", trimmed_path);
        }
    }
}

fn ask_output_file_path() -> String {
    loop {
        print!("\nEntrez le nom du fichier de sortie (sera enregistré dans le dossier 'file_output/') : ");
        io::stdout().flush().unwrap();

        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).unwrap();
        let trimmed_name = file_name.trim();

        if !trimmed_name.is_empty() {
            return format!("file_output/{}", trimmed_name);
        } else {
            println!("Erreur : le nom du fichier ne peut pas être vide.");
        }
    }
}

fn ask_key() -> u8 {
    print!("\nEntrez la clé (nombre entier) : ");
    io::stdout().flush().unwrap();

    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();
    match key.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Clé invalide, clé par défaut = 3.");
            3
        }
    }
}
