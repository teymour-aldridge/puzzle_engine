pub use super::board::{Board, Stone, Point};

/// Represents the result of a Go game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameResult {
    Ongoing,
    Resigned(Stone),
    Finished { black_score: usize, white_score: usize },
}

/// Represents the game state and logic for a game of Go.
#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub to_move: Stone,
    pub result: GameResult,
}

impl Game {
    /// Creates a new Go game with the specified board size.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::game::Game;
    /// let game = Game::new(19);
    /// assert_eq!(game.board.size, 19);
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            board: Board::new(size),
            to_move: Stone::Black,
            result: GameResult::Ongoing,
        }
    }

    /// Attempts to play a move. Returns an error if the move is illegal.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::game::{Game,Point, Stone};
    /// let mut game = Game::new(9);
    /// game.play(Point::new(3, 3)).unwrap();
    /// ```
    pub fn play(&mut self, point: Point) -> Result<(), &'static str> {
        if self.result != GameResult::Ongoing {
            return Err("Game is already over");
        }

        self.board.place_stone(point, self.to_move)?;
        self.to_move = match self.to_move {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        };
        Ok(())
    }

    /// Forfeits the game for the current player.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::game::{Game, Stone};
    /// let mut game = Game::new(9);
    /// game.resign();
    /// assert_eq!(game.result, puzzle_engine::go::game::GameResult::Resigned(Stone::Black));
    /// ```
    pub fn resign(&mut self) {
        self.result = GameResult::Resigned(self.to_move);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initial_state() {
        let game = Game::new(13);
        assert_eq!(game.board.size, 13);
        assert_eq!(game.to_move, Stone::Black);
        assert_eq!(game.result, GameResult::Ongoing);
    }

    #[test]
    fn test_play_alternates_turns() {
        let mut game = Game::new(9);
        assert_eq!(game.to_move, Stone::Black);
        game.play(Point::new(1, 1)).unwrap();
        assert_eq!(game.to_move, Stone::White);
    }

    #[test]
    fn test_resign_ends_game() {
        let mut game = Game::new(9);
        game.resign();
        assert_eq!(game.result, GameResult::Resigned(Stone::Black));
    }

    #[test]
    fn test_cannot_play_after_resign() {
        let mut game = Game::new(9);
        game.resign();
        let result = game.play(Point::new(2, 2));
        assert_eq!(result, Err("Game is already over"));
    }
}
