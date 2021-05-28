#![cfg(test)]

use super::*;

#[test]
fn duplicate_in_row_is_detected() {
    let board = Board::from(&[
        [4, 3, 5, 2, 4, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    assert!(!valid(&board));

    let board = Board::from(&[
        [4, 3, 5, 2, 4, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    assert!(!valid(&board));
}

#[test]
fn duplicate_in_column_is_detected() {
    let board = Board::from(&[
        [4, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [4, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    assert!(!valid(&board));

    let board = Board::from(&[
        [1, 0, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 4
        [1, 0, 0, 0, 0, 0, 0, 0, 0], // row 5
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 7
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 9
    ]);

    assert!(!valid(&board));
}

#[test]
fn duplicate_in_square_is_detected() {
    let board = Board::from(&[
        [4, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 8, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    assert!(!valid(&board));

    let board = Board::from(&[
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
        [1, 0, 0, 0, 0, 0, 0, 0, 0], // row 4
        [0, 0, 1, 0, 0, 0, 0, 0, 0], // row 5
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 7
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 9
    ]);

    assert!(!valid(&board));
}

#[test]
fn valid_board_is_passed() {
    let board = Board::from(&[
        [4, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    assert!(valid(&board));

    let board = Board::from(&[
        [4, 0, 0, 0, 6, 0, 0, 0, 1], // row 1
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 4
        [3, 0, 0, 0, 8, 0, 0, 0, 5], // row 5
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 7
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
        [7, 0, 0, 0, 1, 0, 0, 0, 9], // row 9
    ]);

    assert!(valid(&board));

    let board = Board::from(&[
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 4
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 5
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 7
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 9
    ]);

    assert!(valid(&board));
}

#[test]
fn solves_solvable_puzzles() {
    let board = Board::from(&[
        [0, 3, 5, 2, 0, 9, 7, 8, 0], // row 1
        [6, 0, 2, 5, 0, 1, 4, 0, 3], // row 2
        [1, 9, 0, 8, 0, 4, 0, 6, 2], // row 3
        [8, 2, 6, 0, 0, 0, 3, 4, 7], // row 4
        [3, 7, 4, 6, 0, 2, 9, 1, 5], // row 5
        [9, 5, 1, 0, 0, 0, 6, 2, 8], // row 6
        [5, 1, 0, 3, 0, 6, 0, 7, 4], // row 7
        [2, 0, 8, 9, 0, 7, 1, 0, 6], // row 8
        [0, 6, 3, 4, 1, 8, 2, 5, 0], // row 9
    ]);

    let correct_solution = Board::from(&[
        [4, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    let solution = solve(&board);

    assert_eq!(solution.unwrap(), correct_solution);

    let board = Board::from(&[
        [0, 0, 0, 2, 6, 0, 7, 0, 1], // row 1
        [6, 8, 0, 0, 7, 0, 0, 9, 0], // row 2
        [1, 9, 0, 0, 0, 4, 5, 0, 0], // row 3
        [8, 2, 0, 1, 0, 0, 0, 4, 0], // row 4
        [0, 0, 4, 6, 0, 2, 9, 0, 0], // row 5
        [0, 5, 0, 0, 0, 3, 0, 2, 8], // row 6
        [0, 0, 9, 3, 0, 0, 0, 7, 4], // row 7
        [0, 4, 0, 0, 5, 0, 0, 3, 6], // row 8
        [7, 0, 3, 0, 1, 8, 0, 0, 0], // row 9
    ]);

    let correct_solution = Board::from(&[
        [4, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
        [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
        [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
        [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
        [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
        [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
        [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
        [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
        [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    ]);

    let solution = solve(&board);

    assert_eq!(solution.unwrap(), correct_solution);

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

    let correct_solution = Board::from(&[
        [1, 2, 6, 4, 3, 7, 9, 5, 8], // row 1
        [8, 9, 5, 6, 2, 1, 4, 7, 3], // row 2
        [3, 7, 4, 9, 8, 5, 1, 2, 6], // row 3
        [4, 5, 7, 1, 9, 3, 8, 6, 2], // row 4
        [9, 8, 3, 2, 4, 6, 5, 1, 7], // row 5
        [6, 1, 2, 5, 7, 8, 3, 9, 4], // row 6
        [2, 6, 9, 3, 1, 4, 7, 8, 5], // row 7
        [5, 4, 8, 7, 6, 9, 2, 3, 1], // row 8
        [7, 3, 1, 8, 5, 2, 6, 4, 9], // row 9
    ]);

    let solution = solve(&board);

    assert_eq!(solution.unwrap(), correct_solution);

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

    let correct_solution = Board::from(&[
        [1, 2, 6, 4, 3, 7, 9, 5, 8], // row 1
        [8, 9, 5, 6, 2, 1, 4, 7, 3], // row 2
        [3, 7, 4, 9, 8, 5, 1, 2, 6], // row 3
        [4, 5, 7, 1, 9, 3, 8, 6, 2], // row 4
        [9, 8, 3, 2, 4, 6, 5, 1, 7], // row 5
        [6, 1, 2, 5, 7, 8, 3, 9, 4], // row 6
        [2, 6, 9, 3, 1, 4, 7, 8, 5], // row 7
        [5, 4, 8, 7, 6, 9, 2, 3, 1], // row 8
        [7, 3, 1, 8, 5, 2, 6, 4, 9], // row 9
    ]);

    let solution = solve(&board);

    assert_eq!(solution.unwrap(), correct_solution);
}

#[test]
fn detects_unsolvable_puzzles() {
    let board = Board::from(&[
        [3, 0, 0, 2, 6, 0, 7, 0, 1], // row 1
        [6, 8, 0, 0, 7, 0, 0, 9, 0], // row 2
        [1, 9, 0, 0, 0, 4, 5, 0, 0], // row 3
        [8, 2, 0, 1, 0, 0, 0, 4, 0], // row 4
        [0, 0, 4, 6, 0, 2, 9, 0, 0], // row 5
        [0, 5, 0, 0, 0, 3, 0, 2, 8], // row 6
        [0, 0, 9, 3, 0, 0, 0, 7, 4], // row 7
        [0, 4, 0, 0, 5, 0, 0, 3, 6], // row 8
        [7, 0, 3, 0, 1, 8, 0, 0, 0], // row 9
    ]);

    let solution = solve(&board);

    assert!(solution.is_none());

    let board = Board::from(&[
        [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
        [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
        [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
        [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
        [3, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
    ]);

    let solution = solve(&board);

    assert!(solution.is_none());
}
