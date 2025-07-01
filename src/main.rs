use std::fs;
use std::io::{self, Write};
use ::base64 as base64_lib;
use base64_lib::engine::Engine; // Import the Engine trait for encode()

mod algorithms;
mod types;
use algorithms::*;
use types::{Algo, Mode, SourceType, MultiEncode};

fn main() {
    println!("===== Encodeur/Décodeur Rust =====\n");
    println!("1) Encodage simple");
    println!("2) Décodage simple");
    println!("3) Encodage multiple personnalisé");
    print!("Choisissez une option : ");
    io::stdout().flush().unwrap();

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();

    match choix.trim() {
        "1" => lancer_encodage_ou_decodage(Mode::Encode),
        "2" => lancer_encodage_ou_decodage(Mode::Decode),
        "3" => lancer_pipeline(),
        _ => println!("Option invalide."),
    }
}

fn lancer_encodage_ou_decodage(mode: Mode) {
    let algo = ask_algorithm();
    let source_type = ask_source_type();
    let is_binary = ask_file_mode_if_needed(&source_type);
    let input_text = get_input_text(&source_type, is_binary, mode);


    let key = if matches!(algo, Algo::Cesar | Algo::Xor) {
        ask_key()
    } else {
        3
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
        (Mode::Encode, Algo::Vigenere) => {
            let keyword = ask_keyword();
            vigenere_encode(&input_text, &keyword)
        }
        (Mode::Decode, Algo::Vigenere) => {
            let keyword = ask_keyword();
            vigenere_decode(&input_text, &keyword)
        }
    };

    // Affichage du résultat selon le type de source
    match source_type {
        SourceType::Manual => {
            println!("\nRésultat : {}\n", result);
        }
        SourceType::File => {
            if is_binary {
                println!("\nRésultat (base64) : {}\n", result);
            } else {
                println!("\nRésultat : {}\n", result);
            }
        }
    }

    if let SourceType::File = source_type {
        let output_path = ask_output_file_path();
        if let Err(e) = fs::write(&output_path, result) {
            println!("Erreur lors de l'écriture du fichier : {}", e);
        } else {
            println!("Résultat sauvegardé dans {}", output_path);
        }
    }
}

fn lancer_pipeline() {
    let pipeline = build_pipeline();
    let source_type = ask_source_type();
    let is_binary = ask_file_mode_if_needed(&source_type);
    let input_text = get_input_text(&source_type, is_binary, Mode::Encode);
    let result = apply_pipeline(&pipeline, &input_text);

    println!("\nRésultat :\n{}", result);
    if let SourceType::File = source_type {
        let output_path = ask_output_file_path();
        if let Err(e) = fs::write(&output_path, result) {
            println!("Erreur lors de l'écriture du fichier : {}", e);
        } else {
            println!("Résultat sauvegardé dans {}", output_path);
        }
    }
}

fn ask_algorithm() -> Algo {
    println!("\nListe des algorithmes disponibles :");
    println!("1) César\n2) ROT13\n3) Base64\n4) Hex\n5) Atbash\n6) XOR\n7) Reverse\n8) Binary\n9) Base32\n10) Vigenere");
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
        "10" => Algo::Vigenere,
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

    fn ask_file_path(mode: Mode) -> String {
    loop {
        let dir = match mode {
            Mode::Encode => "encoder",
            Mode::Decode => "decoder",
        };
        print!("\nEntrez le nom du fichier (dans le dossier {}): ", dir);
        io::stdout().flush().unwrap();

        let mut path = String::new();
        io::stdin().read_line(&mut path).unwrap();
        let trimmed_path = path.trim();

        let file_path = format!("{}/{}", dir, trimmed_path);

        if std::path::Path::new(&file_path).exists() {
            return file_path;
        } else {
            println!("Erreur : le fichier '{}' n'existe pas dans le dossier '{}'.", trimmed_path, dir);
        }
    }
}


fn ask_output_file_path() -> String {
    loop {
        print!("\nEntrez le nom du fichier de sortie (sera enregistré dans le dossier 'output/') : ");
        io::stdout().flush().unwrap();

        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name).unwrap();
        let trimmed_name = file_name.trim();

        if !trimmed_name.is_empty() {
            return format!("output/{}", trimmed_name);
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

fn build_pipeline() -> Vec<MultiEncode> {
    print!("\nCombien d'étapes souhaitez-vous dans le pipeline ? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_steps: usize = input.trim().parse().unwrap_or(1);

    let mut pipeline = Vec::new();

    for i in 0..num_steps {
        println!("\nÉtape {} :", i + 1);
        let algo = ask_algorithm();

        let param = match algo {
            Algo::Cesar | Algo::Xor => {
                print!("Clé (entier) : ");
                io::stdout().flush().unwrap();
                let mut p = String::new();
                io::stdin().read_line(&mut p).unwrap();
                Some(p.trim().to_string())
            }
            Algo::Vigenere => {
                print!("Mot-clé : ");
                io::stdout().flush().unwrap();
                let mut p = String::new();
                io::stdin().read_line(&mut p).unwrap();
                Some(p.trim().to_string())
            }
            _ => None,
        };

        pipeline.push(MultiEncode { algo, param });
    }

    pipeline
}

fn apply_pipeline(pipeline: &[MultiEncode], text: &str) -> String {
    let mut current = text.to_string();

    for step in pipeline {
        current = match step.algo {
            Algo::Cesar => {
                let key = step.param.as_ref().unwrap().parse::<u8>().unwrap_or(3);
                caesar_encode(&current, key)
            }
            Algo::Xor => {
                let key = step.param.as_ref().unwrap().parse::<u8>().unwrap_or(3);
                xor_encode(&current, key)
            }
            Algo::Vigenere => {
                let keyword = step.param.as_ref().unwrap();
                vigenere_encode(&current, keyword)
            }
            Algo::Rot13 => rot13_encode(&current),
            Algo::Base64 => base64_encode(&current),
            Algo::Hex => hex_encode(&current),
            Algo::Atbash => atbash_encode(&current),
            Algo::Reverse => reverse_encode(&current),
            Algo::Binary => binary_encode(&current),
            Algo::Base32 => base32_encode(&current),
        };
    }

    current
}


fn ask_keyword() -> String {
    print!("\nEntrez le mot-clé (lettres uniquement) : ");
    io::stdout().flush().unwrap();

    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).unwrap();
    keyword.trim().to_string()
}

fn ask_file_mode_if_needed(source_type: &SourceType) -> bool {
    if let SourceType::File = source_type {
        print!("Fichier texte (1) ou binaire (2) ? : ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim() == "2";
    }
    false
}

fn get_input_text(source_type: &SourceType, binary: bool, mode: Mode) -> String {
    match source_type {
        SourceType::Manual => ask_text(),
        SourceType::File => {
            let file_path = ask_file_path(mode);
            if binary {
                match fs::read(&file_path) {
                    Ok(bytes) => base64_lib::engine::general_purpose::STANDARD.encode(&bytes),
                    Err(_) => {
                        println!("Erreur lors de la lecture du fichier binaire.");
                        String::new()
                    }
                }
            } else {
                match fs::read_to_string(&file_path) {
                    Ok(content) => content.trim().to_string(),
                    Err(_) => {
                        println!("Erreur lors de la lecture du fichier texte.");
                        String::new()
                    }
                }
            }
        }
    }
}
