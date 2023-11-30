# Sudoku solver library for Rust

![Test results](https://github.com/ctamblyn/sudoku-solver/actions/workflows/quickstart.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/sudoku-solver)](https://crates.io/crates/sudoku-solver)
[![Documentation](https://docs.rs/sudoku-solver/badge.svg)](https://docs.rs/sudoku-solver)

This library provides a very simple backtracking algorithm for solving sudoku puzzles.

## Examples

The `solve()` function will yield the first solution found for a given puzzle,
or `None` if no solution exists:

```rust
use sudoku_solver::*;

fn main() {
    let board = Board::from(&[
        [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
        [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
        [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
        [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
        [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
    ]);

    println!("Puzzle:\n{}\n", board);

    if let Some(solution) = solve(&board) {
        println!("Solution:\n{}\n", solution);
    } else {
        println!("No solution found.");
    }
}
```

If a puzzle has multiple solutions and you want to iterate over them, you can use
`SolutionIter`:

```rust
use sudoku_solver::*;

fn main() {
    let board = Board::from(&[
        [9, 0, 6, 0, 7, 0, 4, 0, 3], // row 1
        [0, 0, 0, 4, 0, 0, 2, 0, 0], // row 2
        [0, 7, 0, 0, 2, 3, 0, 1, 0], // row 3
        [5, 0, 0, 0, 0, 0, 1, 0, 0], // row 4
        [0, 4, 0, 2, 0, 8, 0, 6, 0], // row 5
        [0, 0, 3, 0, 0, 0, 0, 0, 5], // row 6
        [0, 3, 0, 7, 0, 0, 0, 5, 0], // row 7
        [0, 0, 7, 0, 0, 5, 0, 0, 0], // row 8
        [4, 0, 5, 0, 1, 0, 7, 0, 8], // row 9
    ]);

    for solution in SolutionIter::new(&board) {
        println!("Solution:\n{}\n", solution);
    }
}
```

## Minimum supported Rust version (MSRV) policy

`sudoku-solver`'s current minimum supported Rust version (MSRV) is **1.60.0**.

`sudoku-solver` is guaranteed to compile with that version.  It might also
compile with older versions, but that could change in a future patch release.

If the MSRV of `sudoku-solver` changes, that will be done in a _minor_ version
release (e.g. 0.6.x -> 0.7.0).
