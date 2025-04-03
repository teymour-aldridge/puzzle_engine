/// Shift a single character by `amount`, wrapping A-Z.
/// Non-alphabetic characters are returned unchanged.
pub fn shift_char(c: char, amount: u8) -> char {
    if c.is_ascii_uppercase() {
        (((c as u8 - b'A' + amount) % 26) + b'A') as char
    } else if c.is_ascii_lowercase() {
        (((c as u8 - b'a' + amount) % 26) + b'a') as char
    } else {
        c
    }
}