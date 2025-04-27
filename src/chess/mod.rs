//! # Puzzle Engine - Chess Module
//!
//! This module provides an in-memory, rule-validating chess engine supporting:
//! - Board initialization and resets
//! - Piece movement validation
//! - Check and Checkmate detection
//! - Display rendering
//!
//! Designed for building AI, puzzles, game servers, or educational tools.
//!
//! # Modules
//! - `board` — Board representation and game logic.
//! - `piece` — Piece definitions and types.
//! - `position` — Board position handling.
//! 
pub mod board;
pub mod piece;
pub mod position;

pub use board::Board;
pub use position::Position;
pub use piece::{Color, PieceType, Piece};
pub use board::GameState;
