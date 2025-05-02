// Déclare les modules
pub mod caesar;
pub mod rot13;
pub mod base64;
pub mod hex;
pub mod atbash;
pub mod xor;
pub mod reverse;
pub mod binary;
pub mod base32;
pub mod vigenere;


// Réexporte leurs fonctions principales pour accès facile
pub use caesar::{encode as caesar_encode, decode as caesar_decode};
pub use rot13::{encode as rot13_encode, decode as rot13_decode};
pub use base64::{encode as base64_encode, decode as base64_decode};
pub use hex::{encode as hex_encode, decode as hex_decode};
pub use atbash::{encode as atbash_encode, decode as atbash_decode};
pub use xor::{encode as xor_encode, decode as xor_decode};
pub use reverse::{encode as reverse_encode, decode as reverse_decode};
pub use binary::{encode as binary_encode, decode as binary_decode};
pub use base32::{encode as base32_encode, decode as base32_decode};
pub use vigenere::{encode as vigenere_encode, decode as vigenere_decode};