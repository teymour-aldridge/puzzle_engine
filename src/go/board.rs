use std::collections::HashMap;

/// Represents a point on the Go board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Create a new [`Point`] with 0-based coordinates.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::board::Point;
    /// let p = Point::new(3, 4);
    /// assert_eq!(p.x, 3);
    /// ```
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Enum for the two players' stones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
}

/// Represents the Go board state.
#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    grid: HashMap<Point, Stone>,
}

impl Board {
    /// Create a new empty Go board of a given size.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::board::Board;
    /// let board = Board::new(9);
    /// assert_eq!(board.size, 9);
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: HashMap::new(),
        }
    }

    /// Returns the stone at the given point, if any.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::board::{Board, Point, Stone};
    /// let mut board = Board::new(9);
    /// board.place_stone(Point::new(2, 2), Stone::Black);
    /// assert_eq!(board.get(Point::new(2, 2)), Some(Stone::Black));
    /// ```
    pub fn get(&self, point: Point) -> Option<Stone> {
        self.grid.get(&point).cloned()
    }

    /// Places a stone on the board.
    ///
    /// Returns `Err` if the position is already occupied.
    ///
    /// # Examples
    /// ```
    /// use puzzle_engine::go::board::{Board, Point, Stone};
    /// let mut board = Board::new(9);
    /// board.place_stone(Point::new(1, 1), Stone::White).unwrap();
    /// ```
    pub fn place_stone(&mut self, point: Point, stone: Stone) -> Result<(), &'static str> {
        if self.grid.contains_key(&point) {
            return Err("Point already occupied");
        }
        self.grid.insert(point, stone);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new(19);
        assert_eq!(board.size, 19);
    }

    #[test]
    fn test_place_and_get_stone() {
        let mut board = Board::new(9);
        let p = Point::new(0, 0);
        board.place_stone(p, Stone::Black);
        assert_eq!(board.get(p), Some(Stone::Black));
    }

    #[test]
    fn test_place_stone_twice_should_error() {
        let mut board = Board::new(9);
        let p = Point::new(2, 2);
        board.place_stone(p, Stone::White).unwrap();
        let result = board.place_stone(p, Stone::Black);
        assert_eq!(result, Err("Point already occupied"));
    }
} 
