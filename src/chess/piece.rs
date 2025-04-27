/// The color of a chess piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

/// The type of a chess piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// A chess piece with type and color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}