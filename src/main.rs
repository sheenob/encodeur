use clap::{Parser, Subcommand};

/// Simple encodeur/décodeur César
#[derive(Parser)]
#[command(name = "encodeur")]
#[command(about = "Encode ou décode un message avec le chiffrement César", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode un message
    Encode {
        /// Texte à encoder
        #[arg(short, long)]
        text: String,

        /// Clé de décalage (ex: 3)
        #[arg(short, long, default_value_t = 3)]
        key: u8,
    },
    /// Décode un message
    Decode {
        /// Texte à décoder
        #[arg(short, long)]
        text: String,

        /// Clé de décalage (ex: 3)
        #[arg(short, long, default_value_t = 3)]
        key: u8,
    },
}

fn caesar_cipher(text: &str, key: u8, encode: bool) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = if encode { key } else { 26 - key };
                (((c as u8 - first + offset) % 26) + first) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text, key } => {
            let result = caesar_cipher(&text, key, true);
            println!("Texte encodé: {}", result);
        }
        Commands::Decode { text, key } => {
            let result = caesar_cipher(&text, key, false);
            println!("Texte décodé: {}", result);
        }
    }
}
