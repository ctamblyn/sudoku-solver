//! Sudoku solving routines.

#[cfg(test)]
mod tests;

use super::board::*;

/// Test whether a sudoku board state obeys the contraints of the game.
///
/// The constraints are:
///
/// * No digit 1-9 is repeated in any given row, column or square.
///
/// Note that zeroes repesent unfilled cells, and do not count as duplicates.
///
/// ## Example
///
/// A board with a duplicate in the first column:
///
/// ```rust
/// # fn main() {
/// # use sudoku_solver::*;
/// let board = Board::from(&[[0u8; BOARD_SIZE]; BOARD_SIZE])
///     .with_cell(0, 0, 9)
///     .with_cell(0, 5, 9);
///
/// assert!(!valid(&board));
/// # }
/// ```
pub fn valid(b: &Board) -> bool {
    const PRECALC_MASKS: [u64; BOARD_SIZE + 1] = [
        0x00_0000_0001,
        0x00_0000_0010,
        0x00_0000_0100,
        0x00_0000_1000,
        0x00_0001_0000,
        0x00_0010_0000,
        0x00_0100_0000,
        0x00_1000_0000,
        0x01_0000_0000,
        0x10_0000_0000,
    ];

    // Check rows.
    for y in 0..BOARD_SIZE {
        let mut acc = 0;

        for x in 0..BOARD_SIZE {
            acc += PRECALC_MASKS[b.get_cell(x, y) as usize];
        }

        if (acc & 0xee_eeee_eee0) != 0 {
            return false;
        }
    }

    // Check columns.
    for x in 0..BOARD_SIZE {
        let mut acc = 0;

        for y in 0..BOARD_SIZE {
            acc += PRECALC_MASKS[b.get_cell(x, y) as usize];
        }

        if (acc & 0xee_eeee_eee0) != 0 {
            return false;
        }
    }

    // Check squares.
    for square in 0..BOARD_SIZE {
        let mut acc = 0;

        let x = SQUARE_SIZE * (square / SQUARE_SIZE);
        let y = SQUARE_SIZE * (square % SQUARE_SIZE);

        for i in 0..BOARD_SIZE {
            acc += PRECALC_MASKS[b.get_cell(x + (i / 3), y + (i % 3)) as usize];
        }

        if (acc & 0xee_eeee_eee0) != 0 {
            return false;
        }
    }

    true
}

fn valid_choices_for_cell(b: &Board, x: usize, y: usize, cutoff_count: usize) -> (usize, u16) {
    let mut count = 0;
    let mut cs = 0;

    for v in 1..=BOARD_SIZE as u8 {
        if valid(&b.with_cell(x, y, v)) {
            count += 1;
            if count >= cutoff_count {
                cs = 0;
                break;
            }

            cs |= 1 << v;
        }
    }

    (count, cs)
}

fn real_solve(b: &Board) -> Option<Board> {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_candidates = 0;
    let mut min_count = BOARD_SIZE + 1;

    // Find the cell with the least number of possible valid values.
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if b.get_cell(x, y) == 0 {
                let (count, cs) = valid_choices_for_cell(b, x, y, min_count);

                if count == 0 {
                    // No valid choices for this empty cell, so we need to backtrack.
                    return None;
                }

                if cs != 0 {
                    min_x = x;
                    min_y = y;
                    min_candidates = cs;
                    min_count = count;
                }
            }
        }
    }

    if min_count == BOARD_SIZE + 1 {
        // No cells can be updated, but the board is valid, so it must be solved.
        return Some(*b);
    }

    // Try all the possible values for the selected cell.
    for v in 1..=BOARD_SIZE as u8 {
        if (min_candidates & (1 << v)) != 0 {
            let b2 = real_solve(&b.with_cell(min_x, min_y, v));
            if b2.is_some() {
                return b2;
            }
        }
    }

    // Nothing worked.
    None
}

/// Solve a sudoku puzzle.
///
/// Returns an `Option<Board>` which is either `None`, if no solution could be found, or a `Some`
/// variant wrapping the first solution found.
///
/// ## Example
///
/// ```rust
/// # fn main() {
/// # use sudoku_solver::*;
/// let board = Board::from(&[
///     [0, 0, 0, 2, 6, 0, 7, 0, 1], // row 1
///     [6, 8, 0, 0, 7, 0, 0, 9, 0], // row 2
///     [1, 9, 0, 0, 0, 4, 5, 0, 0], // row 3
///     [8, 2, 0, 1, 0, 0, 0, 4, 0], // row 4
///     [0, 0, 4, 6, 0, 2, 9, 0, 0], // row 5
///     [0, 5, 0, 0, 0, 3, 0, 2, 8], // row 6
///     [0, 0, 9, 3, 0, 0, 0, 7, 4], // row 7
///     [0, 4, 0, 0, 5, 0, 0, 3, 6], // row 8
///     [7, 0, 3, 0, 1, 8, 0, 0, 0], // row 9
/// ]);
///
/// assert!(solve(&board).is_some());
/// # }
/// ```
pub fn solve(b: &Board) -> Option<Board> {
    if valid(b) {
        real_solve(b)
    } else {
        None
    }
}