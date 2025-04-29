# Encodeur / Décodeur en Rust

Projet d'application Rust permettant d'encoder et de décoder du texte en utilisant plusieurs algorithmes simples.

---

## ✨ Fonctionnalités

- **César** (avec clé de décalage personnalisée)
- **ROT13** (chiffrement symétrique à décalage fixe)
- **Base64** (encodage standard base64)
- **Hexadécimal** (représentation texte en hexadécimal)
- **Atbash** (chiffrement par inversion alphabétique)
- **XOR**    (xor bit à bit)
- **Reverse** (inversion du texte)
- **Binary**  (texte binaire)
- **Base32**  (encodage base32)

---

## 📦 Installation

Assurez-vous d'avoir **Rust** et **Cargo** installés sur votre machine.  
Si ce n'est pas encore fait, installez-les avec :

sudo apt install rustc cargo


**Clonez ce dépôt :**
git clone https://github.com/sheenob/encodeur.git




###########################    GUIDE UTILISATION     ###########################

# Lancer le script

-> cargo run

# Liste des algorithmes disponibles

    Algorithme | Besoin de --key ?  | Notes

    - caesar   | Oui (ex: 4)        | Chiffrement par décalage
    - rot13    | Non                | Déviation fixe de 13 lettres
    - base64   | Non                | Encodage standard ASCII ↔ Base64
    - hex      | Non                | Encodage texte → hexadécimal
    - atbash   | Non                | Inversion alphabétique
    - xor      | Oui (clé numérique)| XOR bit à bit
    - reverse  | Non                | Inverser le texte
    - binary   | Non                | Texte -> 0 et 1
    - base32   | Non                | Encodage standard ASCII ↔ Base32