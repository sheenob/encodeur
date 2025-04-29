use clap::ValueEnum;

/// Liste des algorithmes disponibles.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Algo {
    Cesar,
    Rot13,
    Base64,
    Hex,
    Atbash,
}

/// Mode d'utilisation : encoder ou décoder.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Encode,
    Decode,
}
