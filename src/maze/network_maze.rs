use rand::prelude::*;
use rand::rng;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug)]
pub struct Maze {
    pub start: NodeId,
    pub end: NodeId,
    pub current: NodeId,
    pub graph: HashMap<NodeId, Vec<NodeId>>,
}

#[derive(Debug)]
pub enum MazeError {
    TooFewNodes,
}

impl Maze {
    /// Creates a new randomly generated maze with a specified number of nodes.
    /// 
    /// # Examples
    /// ```
    /// let maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// println!("Start: {:?}, End: {:?}", maze.start, maze.end);
    /// ```
    pub fn new(num_nodes: usize) -> Result<Self, MazeError> {
        if num_nodes < 2 {
            return Err(MazeError::TooFewNodes);
        }

        let mut rng = rng();
        let mut graph: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

        // Ensure all nodes are connected: build a random spanning tree first
        let mut nodes: Vec<NodeId> = (0..num_nodes).map(NodeId).collect();
        nodes.shuffle(&mut rng);
        for i in 1..nodes.len() {
            let a = nodes[i];
            let b = nodes[rng.random_range(0..i)];
            graph.entry(a).or_default().push(b);
            graph.entry(b).or_default().push(a);
        }

        // Add a few random edges
        let extra_edges = num_nodes / 3;
        for _ in 0..extra_edges {
            let a = NodeId(rng.random_range(0..num_nodes));
            let b = NodeId(rng.random_range(0..num_nodes));
            if a != b && !graph.get(&a).map_or(false, |v| v.contains(&b)) {
                graph.entry(a).or_default().push(b);
                graph.entry(b).or_default().push(a);
            }
        }

        let start = NodeId(0);
        let end = NodeId(num_nodes - 1);
        let current = start;

        Ok(Maze { start, end, current, graph })
    }

    /// Returns the neighbors of the given node.
    ///
    /// # Examples
    /// ```
    /// let maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// let neighbors = maze.neighbors(maze.start);
    /// println!("Neighbors of start: {:?}", neighbors);
    /// ```
    pub fn neighbors(&self, node: NodeId) -> &[NodeId] {
        self.graph.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Finds a path from the start node to the end node using BFS.
    /// Returns `Some(Vec<NodeId>)` if a path exists, or `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// let maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// if let Some(path) = maze.find_path() {
    ///     println!("Path found: {:?}", path);
    /// }
    /// ```
    pub fn find_path(&self) -> Option<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();

        visited.insert(self.start);
        queue.push_back(self.start);

        while let Some(current) = queue.pop_front() {
            if current == self.end {
                let mut path = vec![current];
                while let Some(&prev) = came_from.get(&path[path.len() - 1]) {
                    path.push(prev);
                }
                path.reverse();
                return Some(path);
            }
            for &neighbor in self.neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    came_from.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    /// Attempts to move from the current node to a neighboring node.
    /// Returns `Ok(new_node)` if the move is valid, or an `Err` with a message otherwise.
    ///
    /// # Examples
    /// ```
    /// let mut maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// let current = maze.current;
    /// let neighbors = maze.neighbors(current);
    /// if let Some(&next) = neighbors.first() {
    ///     maze.traverse(next).unwrap();
    /// }
    /// ```
    pub fn traverse(&mut self, next: NodeId) -> Result<NodeId, String> {
        if self.neighbors(self.current).contains(&next) {
            self.current = next;
            Ok(self.current)
        } else {
            Err(format!("Cannot move from {:?} to {:?}: not a neighbor", self.current, next))
        }
    }

    /// Returns a simple textual visualization of the maze graph.
    /// Each node is listed with its connections.
    ///
    /// # Examples
    /// ```
    /// let maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// println!("{}", maze.visualize());
    /// ```
    pub fn visualize(&self) -> String {
        let mut lines = vec![
            format!("Maze Visualization (Start: {:?}, End: {:?}, Current: {:?}):", self.start, self.end, self.current)
        ];
        for (node, neighbors) in &self.graph {
            let neighbor_ids: Vec<String> = neighbors.iter().map(|n| format!("{:?}", n)).collect();
            lines.push(format!("{:?}: {}", node, neighbor_ids.join(", ")));
        }
        lines.join("\n")
    }

    /// Returns true if the current node is the end node.
    ///
    /// # Examples
    /// ```
    /// let mut maze = puzzle_engine::maze::network_maze::Maze::new(10).unwrap();
    /// let path_opt = maze.find_path();
    /// match path_opt {
    ///     Some(path) => {
    ///         for next_node in path.iter().skip(1) {
    ///             maze.traverse(next_node.clone()).unwrap();
    ///         }
    ///         if maze.is_at_end() {
    ///             println!("We made it to the end!");
    ///         }
    ///         else {
    ///             println!("the path led us astray.");
    ///         }
    ///     },
    ///     None => println!("There was no path to the end!"),
    /// }
    /// ```
    pub fn is_at_end(&self) -> bool {
        self.current == self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_creation_and_pathfinding() {
        let maze = Maze::new(20).unwrap();
        let path = maze.find_path();
        assert!(path.is_some(), "A path should exist between start and end");
        let path = path.unwrap();
        assert_eq!(path.first().cloned(), Some(maze.start));
        assert_eq!(path.last().cloned(), Some(maze.end));
    }

    #[test]
    fn test_maze_tarversal_small_maze_finds_end() {
        let mut maze = Maze::new(10).unwrap();
        let path = maze.find_path();
        assert!(path.is_some(), "A path should exist between start and end");
        let path = path.unwrap();
        for next_node in path.iter().skip(1){
            maze.traverse(next_node.clone()).unwrap();
        }
        assert_eq!(maze.is_at_end(), true);
    }

    #[test]
    fn test_maze_tarversal_large_maze_finds_end() {
        let mut maze = Maze::new(100_000).unwrap();
        let path = maze.find_path();
        assert!(path.is_some(), "A path should exist between start and end");
        let path = path.unwrap();
        for next_node in path.iter().skip(1){
            maze.traverse(next_node.clone()).unwrap();
        }
        assert_eq!(maze.is_at_end(), true);
    }
    
    #[test]
    fn test_traverse_valid_and_invalid_moves() {
        let mut maze = Maze::new(10).unwrap();
        let current = maze.current;
        let neighbors = maze.neighbors(current);

        if let Some(&next) = neighbors.first() {
            assert_eq!(maze.traverse(next), Ok(next));
            assert_eq!(maze.current, next);
        }

        let invalid = NodeId(9999);
        assert!(maze.traverse(invalid).is_err());
    }
}
