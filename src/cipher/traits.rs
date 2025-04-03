/// Trait for common cipher operations
pub trait CipherPuzzle {
    /// Encrypt the given plaintext
    fn encrypt(&self, plaintext: &str) -> String;

    /// Decrypt the given ciphertext
    fn decrypt(&self, ciphertext: &str) -> String;

    /// Check whether a guess correctly decrypts the ciphertext
    fn validate_solution(&self, ciphertext: &str, guess: &str) -> bool {
        self.decrypt(ciphertext).eq_ignore_ascii_case(guess)
    }
}