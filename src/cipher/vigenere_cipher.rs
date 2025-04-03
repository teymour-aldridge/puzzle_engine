pub use super::traits::CipherPuzzle;

/// Vigenère Cipher
///
/// Each character is encrypted using a corresponding shift from the keyword.
///
/// ## Example
/// ```rust
/// use puzzle_engine::cipher::vigenere_cipher::Vigenere;
/// use puzzle_engine::cipher::prelude::*;
/// let v = Vigenere::new("KEY");
/// let msg = "ATTACKATDAWN";
/// let encrypted = v.encrypt(msg);
/// assert_eq!(v.decrypt(&encrypted), msg);
/// ```
pub struct Vigenere {
    keyword: Vec<u8>, // letter shifts
}

impl Vigenere {
    /// Create a new Vigenère cipher from a keyword (A-Z only)
    pub fn new(keyword: &str) -> Self {
        let keyword = keyword
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase() as u8 - b'A')
            .collect();
        Self { keyword }
    }
}

impl CipherPuzzle for Vigenere {
    fn encrypt(&self, plaintext: &str) -> String {
        vigenere_transform(plaintext, &self.keyword, false)
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        vigenere_transform(ciphertext, &self.keyword, true)
    }
}



/// Core Vigenère transformation
fn vigenere_transform(text: &str, keyword: &[u8], decrypt: bool) -> String {
    let mut result = String::new();
    let mut key_index = 0;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let is_upper = c.is_uppercase();
            let base = if is_upper { b'A' } else { b'a' };
            let offset = c as u8 - base;
            let key = keyword[key_index % keyword.len()];
            let shift = if decrypt {
                (26 + offset - key) % 26
            } else {
                (offset + key) % 26
            };
            result.push((base + shift) as char);
            key_index += 1;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vigenere_encrypt_upper_key_encrypts_correctly() {
        let v = Vigenere::new("AAAAAAAAAAAAAAAAAAAAAAAAAA");
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }

    #[test]
    fn vigenere_encrypt_key_longer_than_message_encrypts_correctly() {
        let v = Vigenere::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }
    #[test]
    fn vigenere_encrypt_key_shorter_than_message_encrypts_correctly() {
        let v = Vigenere::new("b");
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "BCDEFGHIJKLMNOPQRSTUVWXYZA";
        let encrypted = v.encrypt(plain);
        assert_eq!(expected, encrypted);
    }
    
    #[test]
    fn vigenere_encrypt_decrypt() {
        let v = Vigenere::new("KEY");
        let plain = "Attack at dawn!";
        let encrypted = v.encrypt(plain);
        let decrypted = v.decrypt(&encrypted);
        assert_eq!(decrypted, plain);
    }
}