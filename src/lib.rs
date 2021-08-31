//! # Sudoku solver library
//!
//! This library provides a very simple backtracking algorithm for solving sudoku puzzles.
//!
//! ## Examples
//!
//! The [`solve()`] function will yield the first solution found for a given puzzle, or `None` if
//! no solution exists:
//!
//! ```rust
//! # use sudoku_solver::*;
//! let board = Board::from(&[
//!     [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
//!     [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
//!     [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
//!     [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
//!     [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
//!     [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
//!     [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
//!     [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
//!     [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
//! ]);
//!
//! println!("Puzzle:\n{}\n", board);
//!
//! if let Some(solution) = solve(&board) {
//!     println!("Solution:\n{}\n", solution);
//! } else {
//!     println!("No solution found.");
//! }
//! ```
//!
//! If a puzzle has multiple solutions and you want to iterate over them, you can use
//! [`SolutionIter`]:
//!
//! ```rust
//! # fn main() {
//! # use sudoku_solver::*;
//! let board = Board::from(&[
//!     [9, 0, 6, 0, 7, 0, 4, 0, 3], // row 1
//!     [0, 0, 0, 4, 0, 0, 2, 0, 0], // row 2
//!     [0, 7, 0, 0, 2, 3, 0, 1, 0], // row 3
//!     [5, 0, 0, 0, 0, 0, 1, 0, 0], // row 4
//!     [0, 4, 0, 2, 0, 8, 0, 6, 0], // row 5
//!     [0, 0, 3, 0, 0, 0, 0, 0, 5], // row 6
//!     [0, 3, 0, 7, 0, 0, 0, 5, 0], // row 7
//!     [0, 0, 7, 0, 0, 5, 0, 0, 0], // row 8
//!     [4, 0, 5, 0, 1, 0, 7, 0, 8], // row 9
//! ]);
//!
//! let solutions = SolutionIter::new(&board);
//!
//! assert_eq!(solutions.count(), 2);
//! # }
//! ```

#![doc(html_root_url = "https://docs.rs/sudoku-solver/0.2.2")]

pub mod board;
pub mod solver;

pub use board::*;
pub use solver::*;
