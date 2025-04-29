mod algorithms;
mod types;

use clap::Parser;
use algorithms::*;
use types::{Algo, Mode};

#[derive(Parser)]
#[command(name = "encodeur")]
#[command(about = "Encodeur/Décodeur multi-algorithmes en Rust", long_about = None)]
struct Cli {
    /// Texte à encoder ou décoder
    #[arg(short, long)]
    text: String,

    /// Algorithme utilisé (cesar, rot13, base64, hex, atbash)
    #[arg(short, long, value_enum)]
    algo: Algo,

    /// Mode : encode ou decode
    #[arg(short, long, value_enum)]
    mode: Mode,

    /// Clé pour le chiffrement César (optionnel)
    #[arg(short, long, default_value_t = 3)]
    key: u8,
}

fn main() {
    let cli = Cli::parse();

    let result = match (cli.mode, cli.algo) {
        (Mode::Encode, Algo::Cesar) => caesar_encode(&cli.text, cli.key),
        (Mode::Decode, Algo::Cesar) => caesar_decode(&cli.text, cli.key),
        (Mode::Encode, Algo::Rot13) => rot13_encode(&cli.text),
        (Mode::Decode, Algo::Rot13) => rot13_decode(&cli.text),
        (Mode::Encode, Algo::Base64) => base64_encode(&cli.text),
        (Mode::Decode, Algo::Base64) => base64_decode(&cli.text),
        (Mode::Encode, Algo::Hex) => hex_encode(&cli.text),
        (Mode::Decode, Algo::Hex) => hex_decode(&cli.text),
        (Mode::Encode, Algo::Atbash) => atbash_encode(&cli.text),
        (Mode::Decode, Algo::Atbash) => atbash_decode(&cli.text),
    };

    println!("Résultat : {}", result);
}
