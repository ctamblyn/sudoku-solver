//! # Sudoku solver library
//!
//! This library provides a very simple backtracking algorithm for solving sudoku puzzles.
//!
//! ## Example
//!
//! ```rust
//! use sudoku_solver::*;
//!
//! fn main() {
//!     let board = Board::new(&[
//!         [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
//!         [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
//!         [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
//!         [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
//!         [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
//!         [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
//!         [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
//!         [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
//!         [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
//!     ]);
//!
//!     println!("Puzzle:\n{}\n", board);
//!
//!     if let Some(solution) = solve(&board) {
//!         println!("Solution:\n{}\n", solution);
//!     } else {
//!         println!("No solution found.");
//!     }
//! }
//! ```

pub mod board;
pub mod solver;

pub use board::*;
pub use solver::*;
