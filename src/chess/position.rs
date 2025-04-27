/// Represents a square on the chess board (e.g., E2, A1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub file: char, // 'a' - 'h'
    pub rank: u8,   // 1 - 8
}

impl Position {
    pub fn new(file: char, rank: u8) -> Option<Self> {
        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some(Self { file, rank })
        } else {
            None
        }
    }
}