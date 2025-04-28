use super::piece::{Piece, Color, PieceType};
use super::position::Position;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

/// Represents the chess board.
#[derive(Clone, Debug)]
pub struct Board {
    pub squares: HashMap<Position, Piece>,
    pub turn: Color,
    pub game_state: GameState,

    // Castling state
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
}

/// Represents the current state of a chess game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Checkmate(Color), // The player who is checkmated
    Stalemate,
    Draw, // Optional: add later (repetition, 50-move rule, etc.)
}


impl Board {
    /// Create a new board with the initial chess setup.
    pub fn new() -> Self {
        let mut board = Board {
            squares: HashMap::new(),
            turn: Color::White,
            game_state: GameState::Ongoing,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
        };
        board.reset();
        board
    }

    /// Initializes the board with a custom set of pieces, turn, and game state.
    ///
    /// This method clears any existing pieces and replaces them with the provided ones.
    ///
    /// # Arguments
    ///
    /// - `pieces` — A [`Vec`] of tuples representing the initial board state.
    ///   Each tuple must be:
    ///   - `file`: a `char` ('a' to 'h') indicating the file.
    ///   - `rank`: a `u8` (1 to 8) indicating the rank.
    ///   - `color`: a [`Color`] (White or Black).
    ///   - `kind`: a [`PieceType`] (King, Queen, Rook, Bishop, Knight, Pawn).
    /// - `turn` — The [`Color`] whose turn it will be after setup.
    /// - `game_state` — The [`GameState`] to set for the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::*;
    ///
    /// let mut board = Board::new();
    ///
    /// board.initialize_custom(
    ///     vec![
    ///         ('e', 1, Color::White, PieceType::King),
    ///         ('e', 8, Color::Black, PieceType::King),
    ///         ('d', 5, Color::White, PieceType::Queen),
    ///     ],
    ///     Color::Black,
    ///     GameState::Ongoing,
    /// );
    ///
    /// assert_eq!(board.squares.len(), 3);
    /// assert_eq!(board.turn, Color::Black);
    /// assert_eq!(board.game_state, GameState::Ongoing);
    /// assert!(board.squares.contains_key(&Position::new('e', 1).unwrap()));
    /// ```
    ///
    /// # Notes
    ///
    /// - This method is useful for unit tests, AI setup positions, or puzzle setups.
    /// - Passing invalid positions (invalid files/ranks) will panic due to `.unwrap()` on [`Position::new`].
    /// - Standard gameplay usually uses [`Board::new`] instead.
    ///
    pub fn initialize_custom(
        &mut self,
        pieces: Vec<(char, u8, Color, PieceType)>,
        turn: Color,
        game_state: GameState,
    ) {
        self.squares.clear();
        for (file, rank, color, kind) in pieces {
            self.squares.insert(
                Position::new(file, rank).expect("Invalid position construction"),
                Piece { color, kind },
            );
        }
        self.turn = turn;
        self.game_state = game_state;
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

        // Reset castling
        self.white_can_castle_kingside =  true;
        self.white_can_castle_queenside = true;
        self.black_can_castle_kingside = true;
        self.black_can_castle_queenside = true;
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
    pub fn try_move(&mut self, from: Position, to: Position, promotion: Option<PieceType>) -> Result<(), String> {
        let piece = match self.squares.get(&from).copied() {
            Some(p) => p,
            None => return Err("No piece at starting position.".to_string()),
        };
    
        if piece.color != self.turn {
            return Err("Not your turn.".to_string());
        }
    
        let legal_moves = self.get_legal_moves(from);
        if !legal_moves.contains(&to) {
            return Err("Illegal move.".to_string());
        }
    
        // Special handling: castling
        if piece.kind == PieceType::King {
            let castle_rank = match piece.color {
                Color::White => 1,
                Color::Black => 8,
            };
            if from.rank == castle_rank && from.file == 'e' {
                if to.file == 'g' && to.rank == castle_rank {
                    // Kingside castling
                    return self.try_castle(piece.color, true);
                } else if to.file == 'c' && to.rank == castle_rank {
                    // Queenside castling
                    return self.try_castle(piece.color, false);
                }
            }
        }
        // Clone board and simulate move to check for illegal moves
        let mut clone = self.clone();
        clone.force_move(from, to)?;
    
        // If after the move our king is in check, reject
        if clone.is_in_check(piece.color) {
            return Err("Move would leave king in check.".to_string());
        }
    
        // Move is valid; perform it
        self.force_move(from, to)?;
        // Check if promotion is needed
        if let Some(mut moved_piece) = self.squares.get_mut(&to) {
            if moved_piece.kind == PieceType::Pawn {
                let promote = match moved_piece.color {
                    Color::White if to.rank == 8 => true,
                    Color::Black if to.rank == 1 => true,
                    _ => false,
                };
                if promote {
                    let new_piece = match promotion {
                        Some(PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight) => promotion.unwrap(),
                        None => PieceType::Queen, // Default to queen if not specified
                        _ => return Err("Invalid promotion piece.".to_string()),
                    };
                    moved_piece.kind = new_piece;
                }
            }
        }
        // Switch turn
        self.turn = Self::opponent_color(self.turn);

        // After move, check if opponent is checkmated
        if self.is_checkmate(self.turn) {
            self.game_state = GameState::Checkmate(self.turn);
        }
        else {
            self.game_state = GameState::Ongoing;
        }

        Ok(())
    }

    /// Trys to castle
    fn try_castle(&mut self, color: Color, kingside: bool) -> Result<(), String> {
        let (rank, rook_file, king_from, king_to, rook_to) = match (color, kingside) {
            (Color::White, true) => (1, 'h', Position::new('e', 1).unwrap(), Position::new('g', 1).unwrap(), Position::new('f', 1).unwrap()),
            (Color::White, false) => (1, 'a', Position::new('e', 1).unwrap(), Position::new('c', 1).unwrap(), Position::new('d', 1).unwrap()),
            (Color::Black, true) => (8, 'h', Position::new('e', 8).unwrap(), Position::new('g', 8).unwrap(), Position::new('f', 8).unwrap()),
            (Color::Black, false) => (8, 'a', Position::new('e', 8).unwrap(), Position::new('c', 8).unwrap(), Position::new('d', 8).unwrap()),
        };
    
        // 1. Check permission
        let can_castle = match (color, kingside) {
            (Color::White, true) => self.white_can_castle_kingside,
            (Color::White, false) => self.white_can_castle_queenside,
            (Color::Black, true) => self.black_can_castle_kingside,
            (Color::Black, false) => self.black_can_castle_queenside,
        };
        if !can_castle {
            return Err("Castling not allowed (king or rook has moved)".to_string());
        }
    
        // 2. Check rook exists
        let rook_pos = Position::new(rook_file, rank).unwrap();
        match self.squares.get(&rook_pos) {
            Some(piece) if piece.color == color && piece.kind == PieceType::Rook => {},
            _ => return Err("Rook missing for castling".to_string()),
        }
    
        // 3. Check squares between king and rook are empty
        let files_between: Vec<char> = if kingside { vec!['f', 'g'] } else { vec!['b', 'c', 'd'] };
        for file in files_between.iter() {
            let pos = Position::new(*file, rank).unwrap();
            if self.squares.contains_key(&pos) {
                return Err("Cannot castle: path blocked".to_string());
            }
        }
    
        // 4. Check king is not in check and doesn't cross check
        if self.is_in_check(color) {
            return Err("Cannot castle while in check".to_string());
        }
        let passing_files = if kingside { ['f', 'g'] } else { ['d', 'c'] };
        for file in passing_files.iter() {
            let mut clone = self.clone();
            clone.force_move(king_from, Position::new(*file, rank).unwrap())?;
            if clone.is_in_check(color) {
                return Err("Cannot castle through check".to_string());
            }
        }
    
        // 5. Move king and rook
        self.squares.remove(&king_from).unwrap();
        self.squares.remove(&rook_pos).unwrap();
        self.squares.insert(king_to, Piece { color, kind: PieceType::King });
        self.squares.insert(rook_to, Piece { color, kind: PieceType::Rook });
    
        // 6. Disable future castling
        match color {
            Color::White => {
                self.white_can_castle_kingside = false;
                self.white_can_castle_queenside = false;
            }
            Color::Black => {
                self.black_can_castle_kingside = false;
                self.black_can_castle_queenside = false;
            }
        }
    
        // Switch turn
        self.turn = Self::opponent_color(self.turn);
    
        Ok(())
    }

    /// Returns the color opposite of that which is passed in.
    fn opponent_color(color: Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /// Moves a piece from one square to another without legality checks.
    /// Used internally for simulating moves.
    fn force_move(&mut self, from: Position, to: Position) -> Result<(), String> {
        let piece = match self.squares.remove(&from) {
            Some(p) => p,
            None => return Err("No piece at starting position.".to_string()),
        };
        self.squares.insert(to, piece);
        // Disable castling rights
        if let Some(moved_piece) = self.squares.get(&to) {
            if moved_piece.kind == PieceType::King {
                match moved_piece.color {
                    Color::White => {
                        self.white_can_castle_kingside = false;
                        self.white_can_castle_queenside = false;
                    }
                    Color::Black => {
                        self.black_can_castle_kingside = false;
                        self.black_can_castle_queenside = false;
                    }
                }
            }
            if moved_piece.kind == PieceType::Rook {
                if from.file == 'a' && from.rank == 1 {
                    self.white_can_castle_queenside = false;
                }
                if from.file == 'h' && from.rank == 1 {
                    self.white_can_castle_kingside = false;
                }
                if from.file == 'a' && from.rank == 8 {
                    self.black_can_castle_queenside = false;
                }
                if from.file == 'h' && from.rank == 8 {
                    self.black_can_castle_kingside = false;
                }
            }
        }
        Ok(())
    }
    
    /// Determines whether the player of the given color is currently in check.
    ///
    /// `is_in_check` checks if the king of the specified [`Color`] is under attack
    /// by any opposing piece. A player is considered "in check" if the opponent
    /// could legally capture the player's king on their next move.
    ///
    /// # Arguments
    ///
    /// - `color` — The [`Color`] of the player to check (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    ///
    /// Returns `true` if the player's king is currently under threat (in check),
    /// or `false` if the king is safe.
    ///
    /// # Behavior
    ///
    /// - Locates the king of the specified color on the board.
    /// - Simulates all legal moves for opponent pieces.
    /// - If any opponent move could capture the king, returns `true`.
    /// - If no opponent moves threaten the king, returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::*;
    ///
    /// fn main() {
    ///    let mut board = Board::new();
    ///
    ///    // White: f2 to f3
    ///    board.try_move(Position::new('f', 2).unwrap(), Position::new('f', 3).unwrap()).unwrap();
    ///
    ///    // Black: e7 to e5
    ///    board.try_move(Position::new('e', 7).unwrap(), Position::new('e', 5).unwrap()).unwrap();
    ///
    ///    // White: g2 to g4
    ///    board.try_move(Position::new('g', 2).unwrap(), Position::new('g', 4).unwrap()).unwrap();
    ///
    ///    // Black: Queen d8 to h4 (Check!)
    ///    board.try_move(Position::new('d', 8).unwrap(), Position::new('h', 4).unwrap()).unwrap();
    ///
    ///    // White is now in check
    ///    assert!(board.is_in_check(Color::White));
    ///
    ///    println!("White is in check: {}", board.is_in_check(Color::White));
    ///    }
    /// ```
    ///
    /// # Notes
    ///
    /// - `is_in_check` assumes that both players' kings are present on the board.
    ///   If a king is missing (invalid game state), behavior is undefined.
    /// - This method does not detect checkmate or stalemate.
    ///   It only indicates whether the king is immediately threatened.
    /// - Used internally by [`Board::try_move`] to ensure moves do not leave a player in check.
    ///
    /// # See Also
    ///
    /// - [`Board::is_checkmate`] — Determines whether a player is currently checkmated.
    /// - [`Board::try_move`] — Attempts a move while enforcing that the king cannot move into or remain in check.
    ///
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_pos = match self.squares.iter()
            .find(|(_, piece)| piece.color == color && piece.kind == PieceType::King)
        {
            Some((pos, _)) => pos,
            None => return false, // No king found; technically invalid game state
        };

        for (pos, piece) in &self.squares {
            if piece.color != color {
                let moves = self.get_legal_moves(*pos);
                if moves.contains(king_pos) {
                    return true;
                }
            }
        }

        false
    }

    /// Determines whether the player of the given color is currently checkmated.
    ///
    /// `is_checkmate` checks if the king of the specified [`Color`] is under attack (in check),
    /// and whether there are no legal moves available to escape the threat.
    ///
    /// A player is considered checkmated if:
    /// - Their king is currently in check.
    /// - No sequence of legal moves (for any of their pieces) would remove the check.
    ///
    /// If the player is not in check, or if they can make at least one legal move to avoid check,
    /// this method returns `false`.
    ///
    /// # Arguments
    ///
    /// - `color` — The [`Color`] of the player to check for checkmate (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    ///
    /// Returns `true` if the player is checkmated, otherwise `false`.
    ///
    /// # Behavior
    ///
    /// - Attempts all legal moves for all of the player's pieces.
    /// - Simulates each move and checks whether it would remove the king from check.
    /// - If no legal move exists that avoids check, the player is considered checkmated.
    /// - Does not detect stalemate (where a player has no legal moves but is not in check).
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzle_engine::chess::*;
    ///
    /// fn main() {
    ///     let mut board = Board::new();
    ///
    ///     // Simulate Fool's Mate: fastest checkmate in chess
    ///
    ///     // White: f2 to f3
    ///     board.try_move(Position::new('f', 2).unwrap(), Position::new('f', 3).unwrap()).unwrap();
    ///     // Black: e7 to e5
    ///     board.try_move(Position::new('e', 7).unwrap(), Position::new('e', 5).unwrap()).unwrap();
    ///     // White: g2 to g4
    ///     board.try_move(Position::new('g', 2).unwrap(), Position::new('g', 4).unwrap()).unwrap();
    ///     // Black: Queen to h4 (checkmate)
    ///     board.try_move(Position::new('d', 8).unwrap(), Position::new('h', 4).unwrap()).unwrap();
    ///
    ///     assert!(board.is_checkmate(Color::White));
    ///
    ///     println!("White is checkmated!");
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - `is_checkmate` assumes normal chess rules are followed.  
    ///   If illegal positions are manually created (e.g., missing kings), behavior is undefined.
    /// - This method does not consider stalemate or draw conditions.
    /// - It is recommended to call [`Board::is_in_check`] before calling `is_checkmate` if you need to distinguish between check and checkmate.
    ///
    /// # See Also
    ///
    /// - [`Board::is_in_check`] — Determines whether a player's king is currently under attack.
    /// - [`Board::try_move`] — Attempts a move while enforcing move legality, including avoiding self-checks.
    ///
    /// # Future Extensions
    ///
    /// - Full draw/stalemate detection may be implemented in future versions of the engine.
    ///
    pub fn is_checkmate(&self, color: Color) -> bool {
        if !self.is_in_check(color) {
            return false;
        }
    
        for (from, piece) in &self.squares {
            if piece.color != color {
                continue;
            }
    
            let legal_moves = self.get_legal_moves(*from);
            for to in legal_moves {
                let mut cloned = self.clone();
                if cloned.force_move(*from, to).is_ok() && !cloned.is_in_check(color) {
                    return false;
                }
            }
        }
    
        true
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
                let direction = match piece.color {
                    Color::White => 1, // White pawns move UP
                    Color::Black => -1, // Black pawns move DOWN
                };
            
                // Forward move
                if let Some(forward_pos) = Position::new(from.file, (from.rank as i8 + direction) as u8) {
                    if self.squares.get(&forward_pos).is_none() {
                        moves.push(forward_pos);
            
                        // First move double step
                        if (piece.color == Color::White && from.rank == 2) || (piece.color == Color::Black && from.rank == 7) {
                            if let Some(double_forward) = Position::new(from.file, (from.rank as i8 + 2 * direction) as u8) {
                                if self.squares.get(&double_forward).is_none() {
                                    moves.push(double_forward);
                                }
                            }
                        }
                    }
                }
            
                // Captures
                for &file_offset in &[-1, 1] {
                    let capture_file = ((from.file as u8) as i8 + file_offset) as u8 as char;
                    let capture_rank = (from.rank as i8 + direction) as u8;
                    if let Some(capture_pos) = Position::new(capture_file, capture_rank) {
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
            
                // Castling moves
                let rank = match piece.color {
                    Color::White => 1,
                    Color::Black => 8,
                };
            
                // Only if king is on original square
                if from == Position::new('e', rank).unwrap() {
                    // Kingside castling
                    let kingside_allowed = match piece.color {
                        Color::White => self.white_can_castle_kingside,
                        Color::Black => self.black_can_castle_kingside,
                    };
                    if kingside_allowed {
                        let f_pos = Position::new('f', rank).unwrap();
                        let g_pos = Position::new('g', rank).unwrap();
                        if self.squares.get(&f_pos).is_none() && self.squares.get(&g_pos).is_none() {
                            moves.push(g_pos);
                        }
                    }
            
                    // Queenside castling
                    let queenside_allowed = match piece.color {
                        Color::White => self.white_can_castle_queenside,
                        Color::Black => self.black_can_castle_queenside,
                    };
                    if queenside_allowed {
                        let b_pos = Position::new('b', rank).unwrap();
                        let c_pos = Position::new('c', rank).unwrap();
                        let d_pos = Position::new('d', rank).unwrap();
                        if self.squares.get(&b_pos).is_none() && self.squares.get(&c_pos).is_none() && self.squares.get(&d_pos).is_none() {
                            moves.push(c_pos);
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
                let pos = Position::new(file, rank).expect("Invalid position construction");
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
        let pos = Position::new(file, rank).expect("Invalid position construction");
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
        assert!(board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 4).unwrap(), None).is_ok());

        // Black pawn d7 to d5
        assert!(board.try_move(Position::new('d', 7).unwrap(), Position::new('d', 5).unwrap(), None).is_ok());

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

#[cfg(test)]
mod moves_in_direction_tests {
    use super::*;

    #[test]
    fn test_moves_in_directions_empty_board() {
        let board = Board::new();
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
        let mut board = Board::new();
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
        let mut board = Board::new();
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
        let board = Board::new();
        let from = Position::new('d', 4).unwrap();
        let directions = &[(1, 1)]; // Northeast

        let moves = board.moves_in_directions(from, directions, Color::White);
        let expected = [('e', 5), ('f', 6), ('g', 7)];

        assert_eq!(moves.len(), expected.len());
        for (i, pos) in moves.iter().enumerate() {
            assert_eq!(pos.file, expected[i].0);
            assert_eq!(pos.rank, expected[i].1);
        }
    }

    #[test]
    fn test_moves_off_board_edge_case() {
        let board = Board::new();
        let from = Position::new('h', 8).unwrap();
        let directions = &[(1, 0), (0, 1), (1, 1)]; // All directions that should immediately go off board

        let moves = board.moves_in_directions(from, directions, Color::White);

        assert!(moves.is_empty(), "Should not generate any moves from h8 in these directions");
    }

    #[test]
    fn test_moves_in_multiple_directions() {
        let board = Board::new();
        let from = Position::new('d', 4).unwrap();
        let directions = &[(1, 0), (0, 1)]; // East and North

        let moves = board.moves_in_directions(from, directions, Color::White);

        // d4 east: e4, f4, g4, h4
        // d4 north: d5, d6, d7, d8
        let expected_positions = [
            ('e', 4), ('f', 4), ('g', 4), ('h', 4),
            ('d', 5), ('d', 6), ('d', 7),
        ];

        assert_eq!(moves.len(), expected_positions.len());
        for expected in expected_positions.iter() {
            assert!(moves.contains(&Position::new(expected.0, expected.1).unwrap()));
        }
    }
}
#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn test_not_in_check_starting_position() {
        let board = Board::new();
        assert!(!board.is_in_check(Color::White), "White should not be in check at start.");
        assert!(!board.is_in_check(Color::Black), "Black should not be in check at start.");
    }

    #[test]
    fn test_in_check_simple_rook_attack() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('e', 8, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);

        assert!(board.is_in_check(Color::White), "White king should be in check from black rook.");
    }

    #[test]
    fn test_not_in_check_rook_blocked() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('e', 4, Color::White, PieceType::Pawn), // Blocking pawn
            ('e', 8, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(!board.is_in_check(Color::White), "White king should not be in check (rook is blocked).");
    }

    #[test]
    fn test_in_check_diagonal_bishop_attack() {
        let pieces = vec![
            ('c', 1, Color::White, PieceType::King),
            ('f', 4, Color::Black, PieceType::Bishop),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from bishop on diagonal.");
    }

    #[test]
    fn test_in_check_knight_attack() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('d', 3, Color::Black, PieceType::Knight),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from knight.");
    }

    #[test]
    fn test_in_check_queen_attack() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('e', 5, Color::Black, PieceType::Queen),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from queen along file.");
    }

    #[test]
    fn test_in_check_multiple_threats() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('e', 8, Color::Black, PieceType::Rook),
            ('a', 1, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from two directions.");
    }

    #[test]
    fn test_not_in_check_empty_board() {
        let board = Board::new();
        assert!(!board.is_in_check(Color::White), "Empty board should not cause check (no kings).");
        assert!(!board.is_in_check(Color::Black), "Empty board should not cause check (no kings).");
    }

    #[test]
    fn test_in_check_pawn_attack() {
        let pieces = vec![
            ('e', 4, Color::White, PieceType::King),
            ('d', 5, Color::Black, PieceType::Pawn), // Pawn threatens e4
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from black pawn.");
    }

    #[test]
    fn test_not_in_check_wrong_color_pawn() {
        let pieces = vec![
            ('e', 4, Color::Black, PieceType::King),
            ('d', 5, Color::White, PieceType::Pawn), // White pawn can't threaten Black king backwards
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(!board.is_in_check(Color::Black), "White pawn should not check black king from above.");
    }

    #[test]
    fn test_not_checkmated_at_start() {
        let board = Board::new();
        assert!(!board.is_checkmate(Color::White), "White should not be checkmated at starting position.");
        assert!(!board.is_checkmate(Color::Black), "Black should not be checkmated at starting position.");
    }
}

#[cfg(test)]
mod checkmake_tests {
    use super::*;

    #[test]
    fn test_simple_check_but_not_checkmate() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('e', 8, Color::Black, PieceType::Rook), // Black rook attacking, but king can move
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check.");
        assert!(!board.is_checkmate(Color::White), "White king should not be checkmated (can escape).");
    }

    #[test]
    fn test_simple_checkmate_by_rook_and_king_blocked() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('a', 2, Color::Black, PieceType::Rook),
            ('b', 2, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check.");
        assert!(board.is_checkmate(Color::White), "White king should be checkmated (blocked on all sides).");
    }

    #[test]
    fn test_simple_check_by_rook_and_king_can_take_to_escape() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('a', 2, Color::Black, PieceType::Rook),
            ('b', 3, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check.");
        assert!(!board.is_checkmate(Color::White), "White king should be checkmated (blocked on all sides).");
    }

    #[test]
    fn test_checkmate_by_queen() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('h', 1, Color::Black, PieceType::Queen),
            ('b', 3, Color::Black, PieceType::King),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check from queen.");
        assert!(board.is_checkmate(Color::White), "White king should be checkmated (no escape).");
    }

    #[test]
    fn test_not_checkmate_king_can_escape() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('a', 3, Color::Black, PieceType::Queen),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(board.is_in_check(Color::White), "White king should be in check.");
        assert!(!board.is_checkmate(Color::White), "White king should be able to escape (move).");
    }

    #[test]
    fn test_checkmate_fools_mate() {
        let mut board = Board::new();

        // Simulate Fool's Mate
        board.try_move(Position::new('f', 2).unwrap(), Position::new('f', 3).unwrap(), None).unwrap();
        board.try_move(Position::new('e', 7).unwrap(), Position::new('e', 5).unwrap(), None).unwrap();
        board.try_move(Position::new('g', 2).unwrap(), Position::new('g', 4).unwrap(), None).unwrap();
        board.try_move(Position::new('d', 8).unwrap(), Position::new('h', 4).unwrap(), None).unwrap();

        assert!(board.is_in_check(Color::White), "White should be in check.");
        assert!(board.is_checkmate(Color::White), "White should be checkmated in Fool's Mate.");
    }

    #[test]
    fn test_not_checkmate_not_in_check() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('a', 8, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        assert!(!board.is_in_check(Color::White), "White king should not be in check.");
        assert!(!board.is_checkmate(Color::White), "White king should not be checkmated if not in check.");
    }

    #[test]
    fn test_checkmate_edge_case_empty_board() {
        let board = Board::new();

        // No kings on the board technically — behavior undefined, but should NOT panic
        assert!(!board.is_checkmate(Color::White), "Empty board should not panic or cause checkmate.");
        assert!(!board.is_checkmate(Color::Black), "Empty board should not panic or cause checkmate.");
    }

    #[test]
    fn test_checkmate_smothered_mate() {
        let pieces = vec![
            ('h', 8, Color::Black, PieceType::King),
            ('h', 7, Color::Black, PieceType::Pawn),
            ('g', 7, Color::Black, PieceType::King),
            ('g', 8, Color::Black, PieceType::Rook),
            ('f', 7, Color::White, PieceType::Knight),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        // No kings on the board technically — behavior undefined, but should NOT panic
        assert!(!board.is_checkmate(Color::White), "Empty board should not panic or cause checkmate.");
        assert!(!board.is_checkmate(Color::Black), "Empty board should not panic or cause checkmate.");
    }
}
#[cfg(test)]
mod try_move_tests {
    use super::*;

    #[test]
    fn test_try_move_successful_normal_move() {
        let mut board = Board::new();
        let result = board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 4).unwrap(), None);
        assert!(result.is_ok(), "Expected pawn move from e2 to e4 to succeed.");
        assert_eq!(board.turn, Color::Black, "Turn should switch to Black after move.");
        assert_eq!(board.game_state, GameState::Ongoing, "Game should continue after normal move.");
    }

    #[test]
    fn test_try_move_no_piece_at_start() {
        let mut board = Board::new();
        let result = board.try_move(Position::new('e', 3).unwrap(), Position::new('e', 4).unwrap(), None);
        assert!(result.is_err(), "Expected error when no piece at starting position.");
    }

    #[test]
    fn test_try_move_illegal_move_attempt() {
        let mut board = Board::new();
        let result = board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 5).unwrap(), None); // Illegal: pawn can't jump to e5 directly
        assert!(result.is_err(), "Expected illegal move error for pawn jumping 3 spaces.");
    }

    #[test]
    fn test_try_move_not_players_turn() {
        let mut board = Board::new();
        board.turn = Color::Black;
        let result = board.try_move(Position::new('e', 2).unwrap(), Position::new('e', 4).unwrap(), None);
        assert!(result.is_err(), "Expected error when moving out of turn.");
    }

    #[test]
    fn test_try_move_into_check_disallowed() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('b', 2, Color::Black, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        let result = board.try_move(Position::new('a', 1).unwrap(), Position::new('b', 1).unwrap(), None);
        assert!(result.is_err(), "Expected error when moving into check.");
    }

    #[test]
    fn test_try_move_checkmate_after_move() {
        let pieces = vec![
            ('a', 1, Color::White, PieceType::King),
            ('d', 8, Color::Black, PieceType::Queen),
            ('b', 3, Color::Black, PieceType::King),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        board.turn = Color::Black;
        let result = board.try_move(Position::new('d', 8).unwrap(), Position::new('d', 1).unwrap(), None);
        assert!(result.is_ok(), "Expected queen move to e5 to succeed.");
        
        match board.game_state {
            GameState::Checkmate(Color::White) => {},
            other => panic!("Expected White to be checkmated after move, found {:?}", other),
        }
    }

    #[test]
    fn test_try_move_no_checkmate_if_king_can_escape() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('f', 3, Color::Black, PieceType::Queen),
        ];
        let turn = Color::Black;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        let result = board.try_move(Position::new('f', 3).unwrap(), Position::new('f', 2).unwrap(), None);
        assert!(result.is_ok(), "Expected move to succeed.");
        assert_eq!(board.game_state, GameState::Ongoing, "Game should remain ongoing (king can escape).");
    }

    #[test]
    fn test_try_move_stalemate_placeholder() {
        let board = Board::new();

        assert_eq!(board.game_state, GameState::Ongoing, "Empty board should be ongoing (no stalemate yet).");
    }

    #[test]
    fn test_try_move_fools_mate_checkmate() {
        let mut board = Board::new();

        // Fool's Mate (quickest checkmate in chess)
        board.try_move(Position::new('f', 2).unwrap(), Position::new('f', 3).unwrap(), None).unwrap();
        board.try_move(Position::new('e', 7).unwrap(), Position::new('e', 5).unwrap(), None).unwrap();
        board.try_move(Position::new('g', 2).unwrap(), Position::new('g', 4).unwrap(), None).unwrap();
        board.try_move(Position::new('d', 8).unwrap(), Position::new('h', 4).unwrap(), None).unwrap();

        match board.game_state {
            GameState::Checkmate(Color::White) => {},
            other => panic!("Expected White to be checkmated in Fool's Mate, found {:?}", other),
        }
    }
}
#[cfg(test)]
mod promotion_tests {
    use super::*;
    #[test]
    fn test_pawn_promotion_to_queen() {
        let mut board = Board::new();
        board.squares.clear();

        // White pawn at 7th rank
        board.squares.insert(Position::new('a', 7).unwrap(), Piece { color: Color::White, kind: PieceType::Pawn });
        board.turn = Color::White;

        // Move to 8th rank with promotion
        let from = Position::new('a', 7).unwrap();
        let to = Position::new('a', 8).unwrap();
        board.try_move(from, to, Some(PieceType::Queen)).unwrap();

        let piece = board.squares.get(&to).unwrap();
        assert_eq!(piece.kind, PieceType::Queen);
        assert_eq!(piece.color, Color::White);
    }

    #[test]
    fn test_pawn_promotion_checkmate() {
        let pieces = vec![
            ('h', 8, Color::Black, PieceType::Rook),
            ('h', 7, Color::Black, PieceType::King),
            ('g', 7, Color::Black, PieceType::Pawn),
            ('g', 8, Color::Black, PieceType::Bishop),
            ('g', 8, Color::Black, PieceType::Bishop),
            ('h', 6, Color::Black, PieceType::Rook),
            ('f', 7, Color::White, PieceType::Pawn),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);
        let from = {Position { file: ('f'), rank: (7) }};
        let to = {Position { file: ('f'), rank: (8) }};
        board.try_move(from, to, Some(PieceType::Knight)).unwrap();

        assert_eq!(GameState::Checkmate(Color::Black), board.game_state);
    }
}
#[cfg(test)]
mod castle_tests {
    use super::*;

    #[test]
    fn test_white_kingside_castling() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('h', 1, Color::White, PieceType::Rook),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);

        assert!(board.try_move(Position::new('e', 1).unwrap(), Position::new('g', 1).unwrap(), None).is_ok());

        assert_eq!(board.squares.get(&Position::new('g', 1).unwrap()).unwrap().kind, PieceType::King, "King not at g1");
        assert_eq!(board.squares.get(&Position::new('f', 1).unwrap()).unwrap().kind, PieceType::Rook, "Rook not at f1");
    }
    #[test]
    fn test_white_kingside_castling_with_piece_blocking() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('h', 1, Color::White, PieceType::Rook),
            ('g', 1, Color::White, PieceType::Queen),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);

        assert!(board.try_move(Position::new('e', 1).unwrap(), Position::new('g', 1).unwrap(), None).is_err());
    }

    #[test]
    fn test_white_kingside_castling_with_check_blocking() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('h', 1, Color::White, PieceType::Rook),
            ('g', 5, Color::Black, PieceType::Queen),
        ];
        let turn = Color::White;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);

        assert!(board.try_move(Position::new('e', 1).unwrap(), Position::new('g', 1).unwrap(), None).is_err());
    }

    #[test]
    fn test_black_queenside_castling() {
        let pieces = vec![
            ('e', 8, Color::Black, PieceType::King),
            ('a', 8, Color::Black, PieceType::Rook),
        ];
        let turn = Color::Black;
        let game_state = GameState::Ongoing;
        let mut board = Board::new();
        board.initialize_custom(pieces, turn, game_state);

        assert!(board.try_move(Position::new('e', 8).unwrap(), Position::new('c', 8).unwrap(), None).is_ok());

        assert_eq!(board.squares.get(&Position::new('c', 8).unwrap()).unwrap().kind, PieceType::King);
        assert_eq!(board.squares.get(&Position::new('d', 8).unwrap()).unwrap().kind, PieceType::Rook);
    }
}

#[cfg(test)]
mod get_legal_moves_tests {
    use super::*;

    #[test]
    fn test_pawn_initial_double_move() {
        let pieces = vec![
            ('e', 2, Color::White, PieceType::Pawn),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 2).unwrap());
        assert!(moves.contains(&Position::new('e', 3).unwrap()), "Pawn should move forward 1 square.");
        assert!(moves.contains(&Position::new('e', 4).unwrap()), "Pawn should move forward 2 squares on first move.");
    }

    #[test]
    fn test_pawn_blocked_forward() {
        let pieces = vec![
            ('e', 2, Color::White, PieceType::Pawn),
            ('e', 3, Color::Black, PieceType::Pawn),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 2).unwrap());
        assert!(moves.is_empty(), "Pawn should not move if blocked.");
    }

    #[test]
    fn test_pawn_capture_diagonal() {
        let pieces = vec![
            ('d', 4, Color::White, PieceType::Pawn),
            ('c', 5, Color::Black, PieceType::Pawn),
            ('e', 5, Color::Black, PieceType::Pawn),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('d', 4).unwrap());
        assert!(moves.contains(&Position::new('c', 5).unwrap()), "Pawn should capture diagonally left.");
        assert!(moves.contains(&Position::new('e', 5).unwrap()), "Pawn should capture diagonally right.");
    }

    #[test]
    fn test_knight_movement_open_board() {
        let pieces = vec![
            ('d', 4, Color::White, PieceType::Knight),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('d', 4).unwrap());
        let expected_positions = [
            ('c', 6), ('e', 6),
            ('b', 5), ('f', 5),
            ('b', 3), ('f', 3),
            ('c', 2), ('e', 2),
        ];
        for (file, rank) in expected_positions.iter() {
            assert!(moves.contains(&Position::new(*file, *rank).unwrap()), "Knight move to {}{} missing.", file, rank);
        }
    }

    #[test]
    fn test_rook_movement_clear() {
        let pieces = vec![
            ('d', 4, Color::White, PieceType::Rook),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('d', 4).unwrap());
        let expected_positions = [
            ('d', 5), ('d', 6), ('d', 7), ('d', 8),
            ('d', 3), ('d', 2), ('d', 1),
            ('e', 4), ('f', 4), ('g', 4), ('h', 4),
            ('c', 4), ('b', 4), ('a', 4),
        ];
        for (file, rank) in expected_positions.iter() {
            assert!(moves.contains(&Position::new(*file, *rank).unwrap()), "Rook move to {}{} missing.", file, rank);
        }
    }

    #[test]
    fn test_bishop_movement_clear() {
        let pieces = vec![
            ('d', 4, Color::White, PieceType::Bishop),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('d', 4).unwrap());
        let expected_positions = [
            ('e', 5), ('f', 6), ('g', 7), ('h', 8),
            ('c', 5), ('b', 6), ('a', 7),
            ('e', 3), ('f', 2), ('g', 1),
            ('c', 3), ('b', 2), ('a', 1),
        ];
        for (file, rank) in expected_positions.iter() {
            assert!(moves.contains(&Position::new(*file, *rank).unwrap()), "Bishop move to {}{} missing.", file, rank);
        }
    }

    #[test]
    fn test_queen_movement_clear() {
        let pieces = vec![
            ('d', 4, Color::White, PieceType::Queen),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('d', 4).unwrap());

        assert!(moves.contains(&Position::new('d', 5).unwrap()), "Queen vertical move missing.");
        assert!(moves.contains(&Position::new('e', 5).unwrap()), "Queen diagonal move missing.");
        assert!(moves.contains(&Position::new('h', 4).unwrap()), "Queen horizontal move missing.");
    }

    #[test]
    fn test_king_movement_clear() {
        let pieces = vec![
            ('e', 4, Color::White, PieceType::King),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 4).unwrap());
        let expected_positions = [
            ('d', 4), ('f', 4),
            ('e', 5), ('e', 3),
            ('d', 5), ('f', 5),
            ('d', 3), ('f', 3),
        ];
        for (file, rank) in expected_positions.iter() {
            assert!(moves.contains(&Position::new(*file, *rank).unwrap()), "King move to {}{} missing.", file, rank);
        }
    }

    #[test]
    fn test_king_castling_moves_allowed() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('h', 1, Color::White, PieceType::Rook),
            ('a', 1, Color::White, PieceType::Rook),
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 1).unwrap());
        assert!(moves.contains(&Position::new('g', 1).unwrap()), "Kingside castling move should be allowed.");
        assert!(moves.contains(&Position::new('c', 1).unwrap()), "Queenside castling move should be allowed.");
    }

    #[test]
    fn test_king_castling_blocked_path() {
        let pieces = vec![
            ('e', 1, Color::White, PieceType::King),
            ('h', 1, Color::White, PieceType::Rook),
            ('f', 1, Color::White, PieceType::Knight), // Blocking kingside
        ];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 1).unwrap());
        assert!(!moves.contains(&Position::new('g', 1).unwrap()), "Kingside castling should not be allowed if path is blocked.");
    }

    #[test]
    fn test_get_legal_moves_empty_square() {
        let pieces = vec![];
        let mut board = Board::new();
        board.initialize_custom(pieces, Color::White, GameState::Ongoing);

        let moves = board.get_legal_moves(Position::new('e', 4).unwrap());
        assert!(moves.is_empty(), "No moves should exist for empty square.");
    }
}
