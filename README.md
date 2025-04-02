# puzzle_engine
Here’s a polished and pretty `README.md` for your `puzzle_engine` crate, designed to look great on GitHub and crates.io. It introduces the project, highlights its current features (`grid_maze`), and hints at future extensibility.

---

```markdown
# 🧠 puzzle_engine

[![Crates.io](https://img.shields.io/crates/v/puzzle_engine.svg)](https://crates.io/crates/puzzle_engine)
[![Documentation](https://docs.rs/puzzle_engine/badge.svg)](https://docs.rs/puzzle_engine)
[![License](https://img.shields.io/crates/l/puzzle_engine)](LICENSE)
[![CI](https://github.com/your-username/puzzle_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/puzzle_engine/actions)

> A modular Rust engine for building and solving puzzles — starting with grid-based mazes and ready to grow 🧩

---

## ✨ Overview

**`puzzle_engine`** is a general-purpose puzzle library written in Rust. It's designed with extensibility and clarity in mind — ideal for games, educational tools, or AI challenges.

This crate currently includes support for **grid mazes**, a type of perfect maze generated using randomized DFS. More puzzle types (Sudoku, Nonograms, Word Puzzles, etc.) are coming soon!

---

## 🚀 Features

✅ Procedural maze generation using randomized DFS  
✅ Minimal API to move through and solve mazes  
✅ Fully connected (perfect) mazes — no isolated areas  
✅ Lightweight and dependency-free (except `rand`)  
✅ Built-in test coverage and examples  
✅ Easy to extend with other puzzles in the future

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

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
puzzle_engine = "0.1"
```

---

## 📚 Modules

### 🧱 `grid_maze`

A classic maze represented as a 2D grid. Each cell connects to neighbors via randomized depth-first search.

- `Maze::new(width, height)` — Create a maze
- `maze.try_move(Direction)` — Move the player if there's a path
- `maze.is_at_end()` — Check if the player has reached the goal

---

## 🧪 Tests

Run tests with:

```bash
cargo test
```

Includes checks for:

- Valid movement and boundary logic
- Maze completeness
- Player reaching the end

---

## 🔮 Roadmap

Planned puzzle modules:

- [x] Grid Maze (DFS-based)
- [ ] Sudoku (validator + solver)
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

Built with 🧩 and 💛 by [Your Name](https://github.com/andrewsimsd)
```

---

Let me know if you'd like a matching `Cargo.toml` metadata block, GitHub Actions CI file, or scaffold for the next puzzle type!
