use super::piece::{Piece, Color, PieceType};
use super::position::Position;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

/// Represents the chess board.
pub struct Board {
    pub squares: HashMap<Position, Piece>,
    pub turn: Color,
}

impl Board {
    /// Create a new board with the initial chess setup.
    pub fn new() -> Self {
        let mut board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        board.reset();
        board
    }

    /// Resets the chess board to the standard initial setup.
    ///
    /// This method clears all existing pieces from the board, places all white and black pieces
    /// in their standard starting positions, and sets the turn to [`Color::White`].
    ///
    /// After calling `reset()`, the board will be identical to a new [`Board`] created with [`Board::new()`].
    ///
    /// # Behavior
    ///
    /// - White pieces occupy ranks 1 and 2.
    /// - Black pieces occupy ranks 7 and 8.
    /// - Ranks 3 through 6 are empty.
    /// - It becomes White's turn to move.
    ///
    /// # Examples
    ///
    /// ```
    /// //use puzzle_engine::chess::board::Board;
    /// //use puzzle_engine::chess::puzzle_engine::Position;
    /// use puzzle_engine::chess::*;
    /// 
    /// let mut board = Board::new();
    ///
    /// // Play a move: e2 to e4
    /// board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 4).unwrap()).unwrap();
    ///
    /// // Reset the board back to the initial state
    /// board.reset();
    ///
    /// // After reset, the board should have a white pawn back on e2
    /// let e2_piece = board.squares.get(&Position::new('e', 2).unwrap()).unwrap();
    /// assert_eq!(e2_piece.color, Color::White);
    /// assert_eq!(e2_piece.kind, PieceType::Pawn);
    ///
    /// // Turn should be White
    /// assert_eq!(board.turn, Color::White);
    /// ```
    ///
    /// # Notes
    ///
    /// - `reset()` can be called at any time, regardless of the current board state.
    /// - Calling `reset()` after a mid-game position will discard all progress and start fresh.
    /// - To save game history, it is recommended to track moves separately before calling `reset()`.
    ///
    pub fn reset(&mut self) {
        self.squares.clear();
        self.turn = Color::White;

        // Pawns
        for file in 'a'..='h' {
            self.squares.insert(Position::new(file, 2).unwrap(), Piece { color: Color::White, kind: PieceType::Pawn });
            self.squares.insert(Position::new(file, 7).unwrap(), Piece { color: Color::Black, kind: PieceType::Pawn });
        }

        // Rooks
        self.squares.insert(Position::new('a', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Rook });
        self.squares.insert(Position::new('h', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Rook });
        self.squares.insert(Position::new('a', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Rook });
        self.squares.insert(Position::new('h', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Rook });

        // Knights
        self.squares.insert(Position::new('b', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Knight });
        self.squares.insert(Position::new('g', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Knight });
        self.squares.insert(Position::new('b', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Knight });
        self.squares.insert(Position::new('g', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Knight });

        // Bishops
        self.squares.insert(Position::new('c', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Bishop });
        self.squares.insert(Position::new('f', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Bishop });
        self.squares.insert(Position::new('c', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Bishop });
        self.squares.insert(Position::new('f', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Bishop });

        // Queens
        self.squares.insert(Position::new('d', 1).unwrap(), Piece { color: Color::White, kind: PieceType::Queen });
        self.squares.insert(Position::new('d', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::Queen });

        // Kings
        self.squares.insert(Position::new('e', 1).unwrap(), Piece { color: Color::White, kind: PieceType::King });
        self.squares.insert(Position::new('e', 8).unwrap(), Piece { color: Color::Black, kind: PieceType::King });
    }

    /// Attempts to move a piece from one position to another according to chess rules.
    ///
    /// `try_move` validates that the move is legal based on the piece's movement capabilities,
    /// ensures the correct player is moving, and updates the board state accordingly.
    ///
    /// If the move is illegal, or if it is not the current player's turn, an error is returned
    /// describing why the move was rejected.
    ///
    /// # Arguments
    ///
    /// - `from` — The [`Position`] of the piece to move.
    /// - `to` — The [`Position`] where the piece should move.
    ///
    /// # Returns
    ///
    /// Returns [`Ok(())`] if the move was successful, or [`Err(String)`] if the move was invalid.
    ///
    /// # Behavior
    ///
    /// - Validates that a piece exists at the `from` position.
    /// - Verifies that the piece belongs to the player whose turn it is.
    /// - Computes legal moves for the piece and checks that the `to` position is one of them.
    /// - Updates the board by removing the piece from `from` and placing it at `to`.
    /// - Switches the turn to the opposing player.
    ///
    /// # Errors
    ///
    /// - `"No piece at starting position."` if no piece is found at `from`.
    /// - `"Not your turn."` if a player attempts to move a piece belonging to the opponent.
    /// - `"Illegal move."` if the desired move is not legal for the selected piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::*;
    ///
    /// let mut board = Board::new();
    ///
    /// let from = Position::new('e', 2).unwrap();
    /// let to = Position::new('e', 4).unwrap();
    ///
    /// // Attempt a valid pawn move: e2 -> e4
    /// assert!(board.try_move(from, to).is_ok());
    ///
    /// // Attempt an illegal move: e4 -> e5 (not White's turn)
    /// let illegal_move = board.try_move(Position::new('e', 4).unwrap(), Position::new('e', 5).unwrap());
    /// assert!(illegal_move.is_err());
    /// ```
    ///
    /// # Notes
    ///
    /// - `try_move` does not currently handle special rules such as castling, en passant, or pawn promotion.
    ///   These features can be implemented as extensions.
    /// - This method does not verify check or checkmate conditions; it only enforces basic move legality.
    /// - The method assumes that [`Position::new`] has already validated that the provided positions are on the board.
    ///
    pub fn try_move(&mut self, from: Position, to: Position) -> Result<(), String> {
        let piece = match self.squares.get(&from).copied() {
            Some(piece) => piece,
            None => return Err("No piece at starting position.".to_string()),
        };

        if piece.color != self.turn {
            return Err("Not your turn.".to_string());
        }

        let legal_moves = self.get_legal_moves(from);
        if !legal_moves.contains(&to) {
            return Err("Illegal move.".to_string());
        }

        self.squares.remove(&from);
        self.squares.insert(to, piece);
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        Ok(())
    }

    /// Returns all legal moves for the piece at a given position, based on standard chess rules.
    ///
    /// `get_legal_moves` calculates the set of valid destination squares for the piece located
    /// at the specified [`Position`]. It considers the type of piece, the current board state,
    /// and basic movement rules.
    ///
    /// If there is no piece at the given position, an empty vector is returned.
    ///
    /// # Arguments
    ///
    /// - `from` — The [`Position`] of the piece to query legal moves for.
    ///
    /// # Returns
    ///
    /// A [`Vec<Position>`] containing all squares the piece can legally move to from the `from` position.
    /// If no moves are available (e.g., the piece is blocked or missing), the returned vector will be empty.
    ///
    /// # Behavior
    ///
    /// - If no piece is present at `from`, returns an empty vector.
    /// - Calculates movement patterns based on the piece type:
    ///     - Pawns: Forward movement, captures diagonally.
    ///     - Rooks: Straight-line movement along ranks and files.
    ///     - Bishops: Diagonal movement.
    ///     - Knights: L-shaped jumps.
    ///     - Queens: Combined rook and bishop movement.
    ///     - Kings: One square in any direction.
    /// - Blocks moves into squares occupied by friendly pieces.
    /// - Allows capturing enemy pieces.
    /// - Does not validate check or checkmate conditions (pure movement legality only).
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::*;
    ///
    /// let board = Board::new();
    ///
    /// let from = Position::new('e', 2).unwrap();
    /// let legal_moves = board.get_legal_moves(from);
    ///
    /// // A white pawn on e2 at the start can move to e3 and e4
    /// let expected_moves = vec![
    ///     Position::new('e', 3).unwrap(),
    ///     Position::new('e', 4).unwrap(),
    /// ];
    ///
    /// assert_eq!(legal_moves.len(), 2);
    /// for m in expected_moves {
    ///     assert!(legal_moves.contains(&m), "Missing move to {}{}", m.file, m.rank);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - `get_legal_moves` does **not** detect whether a move would leave the king in check.
    ///   Users must implement additional validation if full game legality (including check avoidance) is required.
    /// - Special moves such as **castling**, **en passant**, and **pawn promotion** are not yet supported.
    /// - Returned moves are based purely on piece movement and board occupancy.
    ///
    /// # Future Extensions
    ///
    /// - Castling, en passant captures, and pawn promotion mechanics can extend the move generation.
    /// - Integration with check detection will enable full legal move filtering.
    ///
    pub fn get_legal_moves(&self, from: Position) -> Vec<Position> {
        let piece = match self.squares.get(&from) {
            Some(piece) => piece,
            None => return vec![],
        };

        let mut moves = Vec::new();

        match piece.kind {
            PieceType::Pawn => {
                let direction: i8 = match piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                let start_rank = match piece.color {
                    Color::White => 2,
                    Color::Black => 7,
                };

                // 1-square forward
                let next_rank = (from.rank as i8 + direction) as u8;
                if let Some(pos) = Position::new(from.file, next_rank) {
                    if !self.squares.contains_key(&pos) {
                        moves.push(pos);

                        // 2-square move if at starting position
                        if from.rank == start_rank {
                            let two_ahead = (from.rank as i8 + 2 * direction) as u8;
                            if let Some(pos2) = Position::new(from.file, two_ahead) {
                                if !self.squares.contains_key(&pos2) {
                                    moves.push(pos2);
                                }
                            }
                        }
                    }
                }

                // Captures (diagonal)
                for df in [-1, 1] {
                    let next_file = (from.file as u8 as i8 + df) as u8 as char;
                    if let Some(capture_pos) = Position::new(next_file, next_rank) {
                        if let Some(target_piece) = self.squares.get(&capture_pos) {
                            if target_piece.color != piece.color {
                                moves.push(capture_pos);
                            }
                        }
                    }
                }
            }

            PieceType::Rook => {
                moves.extend(self.moves_in_directions(from, &[(1,0), (-1,0), (0,1), (0,-1)], piece.color));
            }

            PieceType::Bishop => {
                moves.extend(self.moves_in_directions(from, &[(1,1), (-1,1), (1,-1), (-1,-1)], piece.color));
            }

            PieceType::Queen => {
                moves.extend(self.moves_in_directions(from, &[
                    (1,0), (-1,0), (0,1), (0,-1),
                    (1,1), (-1,1), (1,-1), (-1,-1)
                ], piece.color));
            }

            PieceType::Knight => {
                let knight_moves = [
                    (2, 1), (1, 2), (-1, 2), (-2, 1),
                    (-2, -1), (-1, -2), (1, -2), (2, -1)
                ];

                for (df, dr) in &knight_moves {
                    let next_file = (from.file as u8 as i8 + df) as u8 as char;
                    let next_rank = (from.rank as i8 + dr) as u8;
                    if let Some(pos) = Position::new(next_file, next_rank) {
                        if !self.squares.get(&pos).map_or(false, |p| p.color == piece.color) {
                            moves.push(pos);
                        }
                    }
                }
            }

            PieceType::King => {
                let king_moves = [
                    (1, 0), (-1, 0), (0, 1), (0, -1),
                    (1, 1), (-1, 1), (1, -1), (-1, -1)
                ];

                for (df, dr) in &king_moves {
                    let next_file = (from.file as u8 as i8 + df) as u8 as char;
                    let next_rank = (from.rank as i8 + dr) as u8;
                    if let Some(pos) = Position::new(next_file, next_rank) {
                        if !self.squares.get(&pos).map_or(false, |p| p.color == piece.color) {
                            moves.push(pos);
                        }
                    }
                }
            }
        }

        moves
    }

    /// Move outward in given directions until blocked.
    fn moves_in_directions(&self, from: Position, directions: &[(i8, i8)], color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        for (df, dr) in directions {
            let mut next_file = from.file as u8 as i8;
            let mut next_rank = from.rank as i8;

            loop {
                next_file += df;
                next_rank += dr;

                if !(b'a' as i8 <= next_file && next_file <= b'h' as i8 && 1 <= next_rank && next_rank <= 8) {
                    break;
                }

                let pos = Position {
                    file: next_file as u8 as char,
                    rank: next_rank as u8,
                };

                if let Some(other_piece) = self.squares.get(&pos) {
                    if other_piece.color != color {
                        moves.push(pos); // capture
                    }
                    break; // blocked
                } else {
                    moves.push(pos);
                }
            }
        }

        moves
    }

    /// Displays the current board state as a text-based chessboard in the console.
    ///
    /// `display` prints a simple, human-readable representation of the chess board,
    /// showing the position of all pieces and empty squares. The board is printed
    /// with rank numbers (8 to 1) on the left and file letters (a to h) on the bottom,
    /// consistent with standard chess notation.
    ///
    /// Empty squares are represented with a dot (`.`).
    /// White and black pieces are represented using Unicode chess symbols.
    ///
    /// # Behavior
    ///
    /// - The board is printed with rank 8 at the top and rank 1 at the bottom.
    /// - Each piece is rendered as a distinct Unicode symbol based on its color and type.
    /// - Empty squares are displayed as dots (`.`).
    /// - The console output is intended for human reading and debugging, not for parsing by other programs.
    ///
    /// # Example Output
    ///
    /// ```text
    /// 8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
    /// 7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
    /// 6 .  .  .  .  .  .  .  . 
    /// 5 .  .  .  .  .  .  .  . 
    /// 4 .  .  .  .  .  .  .  . 
    /// 3 .  .  .  .  .  .  .  . 
    /// 2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
    /// 1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
    ///    a  b  c  d  e  f  g  h
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::Board;
    ///
    /// let board = Board::new();
    /// board.display();
    /// ```
    ///
    /// # Notes
    ///
    /// - `display` writes directly to standard output (`stdout`).
    /// - The output format is intended for visualization and debugging purposes only.
    /// - Unicode chess symbols may not display correctly in some terminals that lack full Unicode support.
    /// - This method does not return a value; it purely outputs text to the console.
    ///
    /// # See Also
    ///
    /// - For programmatic access to the board's contents, use the [`Board::squares`] field.
    /// - For custom rendering, consider using [`Board::write_display`] which allows writing to any formatter.
    ///
    pub fn display(&self) {
        let mut output = String::new();
        self.write_display(&mut output).unwrap();
        println!("{}", output);
    }
    
    /// Writes the current board state to a given formatter using text-based chessboard formatting.
    ///
    /// `write_display` generates the same visual representation as [`Board::display`],
    /// but writes the output to any type that implements [`std::fmt::Write`].
    ///
    /// This allows greater flexibility, such as capturing the board's appearance
    /// into a [`String`], writing to a file, or embedding into logs.
    ///
    /// Empty squares are represented by dots (`.`).
    /// Pieces are rendered using Unicode chess symbols, with rank numbers (8 to 1) and file letters (a to h).
    ///
    /// # Arguments
    ///
    /// - `w` — A mutable reference to any type implementing [`std::fmt::Write`] (such as [`String`] or `fmt::Formatter`).
    ///
    /// # Returns
    ///
    /// Returns [`Ok(())`] if all writes succeeded, or [`Err(std::fmt::Error)`] if a write operation failed.
    ///
    /// # Behavior
    ///
    /// - Ranks are printed from 8 (top) to 1 (bottom).
    /// - Each piece is printed as a Unicode character corresponding to its type and color.
    /// - Empty squares are shown as dots (`.`).
    /// - After printing all ranks, the file letters (`a` to `h`) are printed at the bottom.
    /// - No color highlighting or board themes are applied; the output is purely textual.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::Board;
    /// use std::fmt::Write;
    ///
    /// let board = Board::new();
    /// let mut output = String::new();
    ///
    /// board.write_display(&mut output).unwrap();
    ///
    /// ```
    ///
    /// # Notes
    ///
    /// - This method provides the core logic behind [`Board::display`], but does not perform any printing itself.
    /// - Useful for unit testing, logging, or advanced applications where direct control over the output destination is required.
    /// - The output format matches that of [`Board::display`], ensuring consistency.
    ///
    /// # See Also
    ///
    /// - [`Board::display`] — Convenience method to print directly to standard output (`stdout`).
    /// - [`std::fmt::Write`] — Trait used for the output target.
    ///
    pub fn write_display<W: FmtWrite>(&self, w: &mut W) -> std::fmt::Result {
        for rank in (1..=8).rev() {
            write!(w, "{} ", rank)?;
            for file in 'a'..='h' {
                let pos = Position::new(file, rank).unwrap();
                if let Some(piece) = self.squares.get(&pos) {
                    let symbol = match (piece.color, piece.kind) {
                        (Color::White, PieceType::Pawn) => '♙',
                        (Color::White, PieceType::Rook) => '♖',
                        (Color::White, PieceType::Knight) => '♘',
                        (Color::White, PieceType::Bishop) => '♗',
                        (Color::White, PieceType::Queen) => '♕',
                        (Color::White, PieceType::King) => '♔',
                        (Color::Black, PieceType::Pawn) => '♟',
                        (Color::Black, PieceType::Rook) => '♜',
                        (Color::Black, PieceType::Knight) => '♞',
                        (Color::Black, PieceType::Bishop) => '♝',
                        (Color::Black, PieceType::Queen) => '♛',
                        (Color::Black, PieceType::King) => '♚',
                    };
                    write!(w, " {} ", symbol)?;
                } else {
                    write!(w, " . ")?;
                }
            }
            writeln!(w)?;
        }
        writeln!(w, "   a  b  c  d  e  f  g  h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_piece_at(board: &Board, file: char, rank: u8, expected_color: Color, expected_type: PieceType) {
        let pos = Position::new(file, rank).unwrap();
        let piece = board.squares.get(&pos)
            .unwrap_or_else(|| panic!("Expected piece at {}{}", file, rank));
    
        assert_eq!(
            piece.color, expected_color,
            "Piece at {}{} has wrong color. Expected {:?}, got {:?}.",
            file, rank, expected_color, piece.color
        );
    
        assert_eq!(
            piece.kind, expected_type,
            "Piece at {}{} has wrong type. Expected {:?}, got {:?}.",
            file, rank, expected_type, piece.kind
        );
    }

    /// Verifies that there is NO piece at a given position.
    fn verify_empty_at(board: &Board, file: char, rank: u8) {
        let pos = Position::new(file, rank).unwrap();
        assert!(
            !board.squares.contains_key(&pos),
            "Expected no piece at {}{}, but found one.",
            file, rank
        );
    }

    fn setup_board_with_piece(file: char, rank: u8, piece: Piece) -> Board {
        let mut board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        board.squares.insert(Position::new(file, rank).unwrap(), piece);
        board
    }

    #[test]
    fn test_moves_in_directions_empty_board() {
        let board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('d', 4).unwrap();
        let directions = &[(1, 0)]; // East

        let moves = board.moves_in_directions(from, directions, Color::White);
        let expected_files = ['e', 'f', 'g', 'h'];

        assert_eq!(moves.len(), expected_files.len());
        for (i, pos) in moves.iter().enumerate() {
            assert_eq!(pos.file, expected_files[i]);
            assert_eq!(pos.rank, 4);
        }
    }

    #[test]
    fn test_moves_blocked_by_ally() {
        let mut board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('d', 4).unwrap();
        board.squares.insert(Position::new('f', 4).unwrap(), Piece { color: Color::White, kind: PieceType::Pawn });

        let directions = &[(1, 0)]; // East

        let moves = board.moves_in_directions(from, directions, Color::White);
        let expected_files = ['e']; // only up to just before 'f'

        assert_eq!(moves.len(), expected_files.len());
        for (i, pos) in moves.iter().enumerate() {
            assert_eq!(pos.file, expected_files[i]);
            assert_eq!(pos.rank, 4);
        }
    }

    #[test]
    fn test_moves_blocked_by_enemy() {
        let mut board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('d', 4).unwrap();
        board.squares.insert(Position::new('f', 4).unwrap(), Piece { color: Color::Black, kind: PieceType::Pawn });

        let directions = &[(1, 0)]; // East

        let moves = board.moves_in_directions(from, directions, Color::White);
        let expected_files = ['e', 'f']; // can capture on 'f'

        assert_eq!(moves.len(), expected_files.len());
        for (i, pos) in moves.iter().enumerate() {
            assert_eq!(pos.file, expected_files[i]);
            assert_eq!(pos.rank, 4);
        }
    }

    #[test]
    fn test_moves_diagonal() {
        let board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('d', 4).unwrap();
        let directions = &[(1, 1)]; // Northeast

        let moves = board.moves_in_directions(from, directions, Color::White);
        let expected = [('e', 5), ('f', 6), ('g', 7), ('h', 8)];

        assert_eq!(moves.len(), expected.len());
        for (i, pos) in moves.iter().enumerate() {
            assert_eq!(pos.file, expected[i].0);
            assert_eq!(pos.rank, expected[i].1);
        }
    }

    #[test]
    fn test_moves_off_board_edge_case() {
        let board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('h', 8).unwrap();
        let directions = &[(1, 0), (0, 1), (1, 1)]; // All directions that should immediately go off board

        let moves = board.moves_in_directions(from, directions, Color::White);

        assert!(moves.is_empty(), "Should not generate any moves from h8 in these directions");
    }

    #[test]
    fn test_moves_in_multiple_directions() {
        let board = Board {
            squares: HashMap::new(),
            turn: Color::White,
        };
        let from = Position::new('d', 4).unwrap();
        let directions = &[(1, 0), (0, 1)]; // East and North

        let moves = board.moves_in_directions(from, directions, Color::White);

        // d4 east: e4, f4, g4, h4
        // d4 north: d5, d6, d7, d8
        let expected_positions = [
            ('e', 4), ('f', 4), ('g', 4), ('h', 4),
            ('d', 5), ('d', 6), ('d', 7), ('d', 8),
        ];

        assert_eq!(moves.len(), expected_positions.len());
        for expected in expected_positions.iter() {
            assert!(moves.contains(&Position::new(expected.0, expected.1).unwrap()));
        }
    }

  #[test]
    fn test_reset_board_setup() {
        let board = Board::new();

        assert_eq!(board.turn, Color::White);
        assert_eq!(board.squares.len(), 32);

        for file in 'a'..='h' {
            verify_piece_at(&board, file, 2, Color::White, PieceType::Pawn);
            verify_piece_at(&board, file, 7, Color::Black, PieceType::Pawn);
        }

        for &(file, rank) in &[('a', 1), ('h', 1), ('a', 8), ('h', 8)] {
            let color = if rank == 1 { Color::White } else { Color::Black };
            verify_piece_at(&board, file, rank, color, PieceType::Rook);
        }

        for &(file, rank) in &[('b', 1), ('g', 1), ('b', 8), ('g', 8)] {
            let color = if rank == 1 { Color::White } else { Color::Black };
            verify_piece_at(&board, file, rank, color, PieceType::Knight);
        }

        for &(file, rank) in &[('c', 1), ('f', 1), ('c', 8), ('f', 8)] {
            let color = if rank == 1 { Color::White } else { Color::Black };
            verify_piece_at(&board, file, rank, color, PieceType::Bishop);
        }

        verify_piece_at(&board, 'd', 1, Color::White, PieceType::Queen);
        verify_piece_at(&board, 'd', 8, Color::Black, PieceType::Queen);
        verify_piece_at(&board, 'e', 1, Color::White, PieceType::King);
        verify_piece_at(&board, 'e', 8, Color::Black, PieceType::King);
    }

    #[test]
    fn test_en_passant_setup() {
        let mut board = Board::new();

        // White pawn e2 to e4
        assert!(board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 4).unwrap()).is_ok());

        // Black pawn d7 to d5
        assert!(board.try_move(Position::new('d', 7).unwrap(), Position::new('d', 5).unwrap()).is_ok());

        // Now check positions
        verify_piece_at(&board, 'e', 4, Color::White, PieceType::Pawn);
        verify_piece_at(&board, 'd', 5, Color::Black, PieceType::Pawn);

        // (En passant move itself would need full en passant rule implementation later.)
    }
    #[test]
    fn test_display_initial_board() {
        let board = Board::new();
        let mut output = String::new();
        board.write_display(&mut output).unwrap();

        // Check critical pieces individually

        // Black back rank (rank 8)
        assert!(output.contains("8"), "Rank 8 missing");
        assert!(output.contains("♜"), "Black rook missing");
        assert!(output.contains("♞"), "Black knight missing");
        assert!(output.contains("♝"), "Black bishop missing");
        assert!(output.contains("♛"), "Black queen missing");
        assert!(output.contains("♚"), "Black king missing");

        // Black pawns (rank 7)
        assert!(output.contains("7"), "Rank 7 missing");
        assert!(output.matches('♟').count() >= 8, "Not enough black pawns");

        // White pawns (rank 2)
        assert!(output.contains("2"), "Rank 2 missing");
        assert!(output.matches('♙').count() >= 8, "Not enough white pawns");

        // White back rank (rank 1)
        assert!(output.contains("1"), "Rank 1 missing");
        assert!(output.contains("♖"), "White rook missing");
        assert!(output.contains("♘"), "White knight missing");
        assert!(output.contains("♗"), "White bishop missing");
        assert!(output.contains("♕"), "White queen missing");
        assert!(output.contains("♔"), "White king missing");

        // Files printed
        for file in 'a'..='h' {
            assert!(output.contains(file), "File {} missing", file);
        }
    }
}

