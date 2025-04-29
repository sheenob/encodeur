use clap::ValueEnum;

/// Liste des algorithmes disponibles.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Algo {
    Cesar,
    Rot13,
    Base64,
    Hex,
    Atbash,
    Xor,
    Reverse,
    Binary,
    Base32,
}

/// Mode d'utilisation : encoder ou décoder.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Encode,
    Decode,
}

/// Source d'entrée : texte manuel ou fichier
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SourceType {
    Manual,
    File,
}

