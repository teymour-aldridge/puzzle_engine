use rand::seq::SliceRandom;
use std::collections::{HashSet, VecDeque};

/// Represents a 2D position in the maze grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Cardinal directions used to move within the maze.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A maze generated using randomized DFS, with support for traversal.
pub struct Maze {
    width: usize,
    height: usize,
    visited: HashSet<Position>,
    connections: HashSet<(Position, Position)>,
    start: Position,
    end: Position,
    /// The current position of the player within the maze.
    pub player: Position,
}

impl Maze {
    /// Creates a new maze with the given dimensions, generating a path from start to end.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the maze
    /// * `height` - Height of the maze
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_engine;
    /// let maze = puzzle_engine::maze::grid_maze::Maze::new(5, 5);
    /// assert_eq!(maze.player,puzzle_engine::maze::grid_maze::Position { x: 0, y: 0 });
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: width - 1, y: height - 1 };
        let mut maze = Maze {
            width,
            height,
            visited: HashSet::new(),
            connections: HashSet::new(),
            start,
            end,
            player: start,
        };
        maze.generate_iterative();
        maze
    }

    /// Internal function to generate the maze using iterative DFS (depth-first search).
    fn generate_iterative(&mut self) {
        let mut rng = rand::rng();
        let mut stack = VecDeque::new();
        stack.push_back(self.start);
        self.visited.insert(self.start);

        while let Some(pos) = stack.pop_back() {
            let mut directions = [Direction::North, Direction::South, Direction::East, Direction::West];
            directions.shuffle(&mut rng);

            for dir in directions {
                if let Some(next_pos) = self.move_pos(pos, dir) {
                    if !self.visited.contains(&next_pos) {
                        self.connections.insert((pos, next_pos));
                        self.connections.insert((next_pos, pos));
                        self.visited.insert(next_pos);
                        stack.push_back(next_pos);
                    }
                }
            }
        }
    }
    /// internal function that returns the new position if moving from a given position in a certain direction is valid.
    fn move_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::North if pos.y > 0 => Some(Position { x: pos.x, y: pos.y - 1 }),
            Direction::South if pos.y < self.height - 1 => Some(Position { x: pos.x, y: pos.y + 1 }),
            Direction::East if pos.x < self.width - 1 => Some(Position { x: pos.x + 1, y: pos.y }),
            Direction::West if pos.x > 0 => Some(Position { x: pos.x - 1, y: pos.y }),
            _ => None,
        }
    }

    /// Attempts to move the player in the given direction if there is a path.
    ///
    /// # Arguments
    ///
    /// * `dir` - Direction to move
    ///
    /// # Returns
    ///
    /// `true` if the move was successful, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_engine;
    /// let mut maze = puzzle_engine::maze::grid_maze::Maze::new(4, 4);
    /// maze.try_move(puzzle_engine::maze::grid_maze::Direction::East);
    /// ```
    pub fn try_move(&mut self, dir: Direction) -> bool {
        if let Some(new_pos) = self.move_pos(self.player, dir) {
            if self.connections.contains(&(self.player, new_pos)) {
                self.player = new_pos;
                return true;
            }
        }
        false
    }

    /// Checks if the player has reached the end of the maze.
    ///
    /// # Returns
    ///
    /// `true` if the player's current position is the end position.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_engine;
    /// let mut maze = puzzle_engine::maze::grid_maze::Maze::new(3, 3);
    /// // simulate movement...
    /// if maze.is_at_end() {
    ///     println!("Maze solved!");
    /// }
    /// ```
    pub fn is_at_end(&self) -> bool {
        self.player == self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_maze_player_at_start() {
        let maze = Maze::new(5, 5);
        assert_eq!(maze.player, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_move_pos_bounds() {
        let maze = Maze::new(3, 3);
        let pos = Position { x: 1, y: 1 };

        assert_eq!(maze.move_pos(pos, Direction::North), Some(Position { x: 1, y: 0 }));
        assert_eq!(maze.move_pos(pos, Direction::South), Some(Position { x: 1, y: 2 }));
        assert_eq!(maze.move_pos(pos, Direction::East), Some(Position { x: 2, y: 1 }));
        assert_eq!(maze.move_pos(pos, Direction::West), Some(Position { x: 0, y: 1 }));
    }

    #[test]
    fn test_try_move_succeeds() {
        let mut maze = Maze::new(4, 4);
        let original_pos = maze.player;

        let moved = [Direction::East, Direction::South, Direction::North, Direction::West]
            .iter()
            .any(|&dir| maze.try_move(dir));

        assert!(moved);
        assert_ne!(maze.player, original_pos);
    }

    #[test]
    fn test_is_at_end() {
        let mut maze = Maze::new(2, 2);
        maze.player = Position { x: 1, y: 1 };
        assert!(maze.is_at_end());
    }

    #[test]
    fn test_maze_has_all_cells_visited() {
        let maze = Maze::new(3, 3);
        assert_eq!(maze.visited.len(), 9);
    }
}
