# 🧠 puzzle_engine

[![Crates.io](https://img.shields.io/crates/v/puzzle_engine.svg)](https://crates.io/crates/puzzle_engine)
[![Documentation](https://docs.rs/puzzle_engine/badge.svg)](https://docs.rs/puzzle_engine)
[![License](https://img.shields.io/crates/l/puzzle_engine)](LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-Repository-181717?logo=github)](https://github.com/andrewsimsd/puzzle_engine)

A modular Rust engine for building and solving puzzles.

---

## ✨ Overview

**`puzzle_engine`** is a general-purpose puzzle library written in Rust. It's designed with extensibility and clarity in mind — ideal for games, educational tools, or AI challenges.

This crate currently includes support for the following: 
### Mazes 
- **Grid Mazes**, a 2 dimensional maze generated using randomized DFS.
- **Network Mazes**, a type of maze that consists of a randomly generated network of nodes. 
### Ciphers
- **Caesar**, A simple cipher where each letter is shifted by a fixed number of positions in the alphabet.
- **Vigenere**, A simple cipher where each character is encrypted using a corresponding shift from the keyword.

---

## 🚀 Features

✅ Procedural maze generation using randomized DFS  
✅ Minimal API to move through and solve mazes  
✅ Fully connected mazes — no isolated areas  
✅ Built-in test coverage and examples  
✅ Easy to extend with other puzzles in the future
✅ Simple ciphers

---

## 🧩 Example: Grid Maze

```rust
use puzzle_engine::grid_maze::{Maze, Direction};

fn main() {
    let mut maze = Maze::new(5, 5);

    println!("Starting at: {:?}", maze.player);

    if maze.try_move(Direction::East) {
        println!("Moved to: {:?}", maze.player);
    }

    if maze.is_at_end() {
        println!("Maze solved!");
    }
}
```

## 🧩 Example: Network Maze

```rust
use puzzle_engine::network_maze::Maze;

fn main() {
    let mut maze = Maze::new(10).unwrap();
    let path = maze.find_path();
    let path = path.unwrap();
    for next_node in path.iter().skip(1){
        maze.traverse(next_node.clone()).unwrap();
    }
    if maze.is_at_end() {
        println!("Maze solved!");
    }
}
```

## 🧩 Example: Vigenere Cipher

```rust
use puzzle_engine::cipher::vigenere_cipher::Vigenere;
use puzzle_engine::cipher::prelude::*;

fn main() {
    let v = Vigenere::new("KEY");
    let plain = "Attack at dawn!";
    let encrypted = v.encrypt(plain);
    let decrypted = v.decrypt(&encrypted);
     println!("plain: {}, encrypted: {}", plain, encrypted);
}
```

---

## 🔮 Roadmap

Planned puzzle modules:

- [x] Grid Maze (DFS-based)
- [x] Network Maze
- [ ] More Ciphers
- [ ] Nonograms
- [ ] Word search / Crossword generator
- [ ] Sokoban-style logic puzzles
- [ ] Puzzle trait abstraction for polymorphic puzzle engines

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or PRs for new puzzle types, algorithm improvements, tests, or docs.

---

## 📄 License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

See [LICENSE](LICENSE) for details.

---

## 🔗 Links

- [Documentation](https://docs.rs/puzzle_engine)
- [Crate on crates.io](https://crates.io/crates/puzzle_engine)
- [GitHub Repo](https://github.com/andrewsimsd/puzzle_engine)

---

Built with 🧩 and 💛 by [Andrew Sims](https://github.com/andrewsimsd)

