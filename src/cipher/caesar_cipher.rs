
pub use super::traits::CipherPuzzle;
use super::util::shift_char;

/// Caesar Cipher
///
/// Each letter is shifted by a fixed number of positions in the alphabet.
///
/// ## Example
/// ```rust
/// use puzzle_engine::cipher::caesar_cipher::Caesar;
/// use puzzle_engine::cipher::prelude::*;
/// let c = Caesar::new(1);
/// assert_eq!(c.encrypt("ABC"), "BCD");
/// assert_eq!(c.decrypt("BCD"), "ABC");
/// ```
pub struct Caesar {
    shift: u8,
}

impl Caesar {
    /// Create a new Caesar cipher with the given shift (0-25)
    pub fn new(shift: u8) -> Self {
        Self { shift: shift % 26 }
    }
}

impl CipherPuzzle for Caesar {
    fn encrypt(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| shift_char(c, self.shift))
            .collect()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|c| shift_char(c, 26 - self.shift))
            .collect()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_encrypts_correctly() {
        let c = Caesar { shift: 3 };
        let plain = "Hello, World!";
        let expected = "Khoor, Zruog!";
        let encrypted = c.encrypt(plain);
        assert_eq!(encrypted, expected);
    }
    
    #[test]
    fn caesar_upper_with_wrap_encrypts_correctly() {
        let c = Caesar { shift: 3 };
        let plain = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let expected = "DEFGHIJKLMNOPQRSTUVWXYZABC";
        let encrypted = c.encrypt(plain);
        assert_eq!(encrypted, expected);
    }

    #[test]
    fn caesar_lower_with_wrap_encrypts_correctly() {
        let c = Caesar { shift: 3 };
        let plain = "abcdefghijklmnopqrstuvwxyz";
        let expected = "defghijklmnopqrstuvwxyzabc";
        let encrypted = c.encrypt(plain);
        assert_eq!(encrypted, expected);
    }

    #[test]
    fn caesar_encrypt_decrypt() {
        let c = Caesar { shift: 3 };
        let plain = "Hello, World!";
        let encrypted = c.encrypt(plain);
        let decrypted = c.decrypt(&encrypted);
        assert_eq!(decrypted, plain);
    }
}
