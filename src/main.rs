use std::io::{self, Write};
mod algorithms;
mod types;
use algorithms::*;
use types::{Algo, Mode};

fn main() {
    println!("===== Encodeur/Décodeur Rust =====\n");

    let mode = ask_mode();
    let algo = ask_algorithm();
    let text = ask_text();
    let key = if matches!(algo, Algo::Cesar | Algo::Xor) {
        ask_key()
    } else {
        3 // clé par défaut pour les autres algos
    };

    let result = match (mode, algo) {
        (Mode::Encode, Algo::Cesar) => caesar_encode(&text, key),
        (Mode::Decode, Algo::Cesar) => caesar_decode(&text, key),
        (Mode::Encode, Algo::Rot13) => rot13_encode(&text),
        (Mode::Decode, Algo::Rot13) => rot13_decode(&text),
        (Mode::Encode, Algo::Base64) => base64_encode(&text),
        (Mode::Decode, Algo::Base64) => base64_decode(&text),
        (Mode::Encode, Algo::Hex) => hex_encode(&text),
        (Mode::Decode, Algo::Hex) => hex_decode(&text),
        (Mode::Encode, Algo::Atbash) => atbash_encode(&text),
        (Mode::Decode, Algo::Atbash) => atbash_decode(&text),
        (Mode::Encode, Algo::Xor) => xor_encode(&text, key),
        (Mode::Decode, Algo::Xor) => xor_decode(&text, key),
        (Mode::Encode, Algo::Reverse) => reverse_encode(&text),
        (Mode::Decode, Algo::Reverse) => reverse_decode(&text),
        (Mode::Encode, Algo::Binary) => binary_encode(&text),
        (Mode::Decode, Algo::Binary) => binary_decode(&text),
        (Mode::Encode, Algo::Base32) => base32_encode(&text),
        (Mode::Decode, Algo::Base32) => base32_decode(&text),
    };

    println!("\nRésultat : {}\n", result);
}

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

fn ask_text() -> String {
    print!("\nEntrez le texte : ");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();
    text.trim().to_string()
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
