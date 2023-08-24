//! Sudoku solving routines.

#[cfg(test)]
mod tests;

use std::iter::FusedIterator;

use super::board::*;
use bit_iter::BitIter;

/// Test whether a sudoku board state obeys the contraints of the game.
///
/// The constraints are:
///
/// * No digit 1-9 is repeated in any given row, column or square.
/// * Every cell contains a value from 0-9 inclusive.
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
/// let mut board = Board::from(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
/// board.set_cell(0, 0, 9);
/// board.set_cell(0, 5, 9);
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

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if b.get_cell(x, y) > 9 {
                return false;
            }
        }
    }

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

        let x = SQUARE_SIZE * (square % SQUARE_SIZE);
        let y = SQUARE_SIZE * (square / SQUARE_SIZE);

        for i in 0..BOARD_SIZE {
            acc += PRECALC_MASKS[b.get_cell(x + (i % 3), y + (i / 3)) as usize];
        }

        if (acc & 0xee_eeee_eee0) != 0 {
            return false;
        }
    }

    true
}

fn valid_choices_for_cell(b: &Board, x: usize, y: usize) -> u16 {
    let mut cs = 0b00_0000_0001;

    let xs = SQUARE_SIZE * (x / SQUARE_SIZE);
    let ys = SQUARE_SIZE * (y / SQUARE_SIZE);

    // Generate a mask of already-used values.
    for i in 0..BOARD_SIZE {
        cs |= b.get_cell_as_mask(x, i);
        cs |= b.get_cell_as_mask(i, y);
        cs |= b.get_cell_as_mask(xs + (i % 3), ys + (i / 3));
    }

    // Invert the mask to indicate which choices are still available.
    cs ^ 0b11_1111_1111u16
}

fn cell_with_fewest_candidates(b: &Board) -> Option<(usize, usize, u16)> {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_candidates = 0;
    let mut min_count = BOARD_SIZE + 1;

    // Find the cell with the least number of possible valid values.
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if b.get_cell_as_mask(x, y) == 1 {
                let cs = valid_choices_for_cell(b, x, y);

                if cs == 0 {
                    // No valid choices for this empty cell, so we need to backtrack.
                    return None;
                }

                let count = cs.count_ones() as usize;

                if count == 1 {
                    // Can't do better than this.
                    return Some((x, y, cs));
                } else if count < min_count {
                    min_x = x;
                    min_y = y;
                    min_candidates = cs;
                    min_count = count;
                }
            }
        }
    }

    Some((min_x, min_y, min_candidates))
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
    SolutionIter::new(b).next()
}

/// An iterator which produces the set of solutions to a sudoku-style puzzle.
///
/// Strictly speaking, sudokus should have only one solution.  However, it is possible to construct
/// sudoku-style puzzles with multiple solutions.  `SolutionIter` provides a means of generating
/// such solutions lazily.
///
/// ## Example
///
/// ```rust
/// # fn main() {
/// # use sudoku_solver::*;
/// let mut solutions = SolutionIter::new(&Board::from(&[
///     [9, 0, 6, 0, 7, 0, 4, 0, 3], // row 1
///     [0, 0, 0, 4, 0, 0, 2, 0, 0], // row 2
///     [0, 7, 0, 0, 2, 3, 0, 1, 0], // row 3
///     [5, 0, 0, 0, 0, 0, 1, 0, 0], // row 4
///     [0, 4, 0, 2, 0, 8, 0, 6, 0], // row 5
///     [0, 0, 3, 0, 0, 0, 0, 0, 5], // row 6
///     [0, 3, 0, 7, 0, 0, 0, 5, 0], // row 7
///     [0, 0, 7, 0, 0, 5, 0, 0, 0], // row 8
///     [4, 0, 5, 0, 1, 0, 7, 0, 8], // row 9
/// ]));
///
/// assert_eq!(solutions.next(), Some(Board::from(&[
///     [9, 2, 6, 5, 7, 1, 4, 8, 3], // row 1
///     [3, 5, 1, 4, 8, 6, 2, 7, 9], // row 2
///     [8, 7, 4, 9, 2, 3, 5, 1, 6], // row 3
///     [5, 8, 2, 3, 6, 7, 1, 9, 4], // row 4
///     [1, 4, 9, 2, 5, 8, 3, 6, 7], // row 5
///     [7, 6, 3, 1, 4, 9, 8, 2, 5], // row 6
///     [2, 3, 8, 7, 9, 4, 6, 5, 1], // row 7
///     [6, 1, 7, 8, 3, 5, 9, 4, 2], // row 8
///     [4, 9, 5, 6, 1, 2, 7, 3, 8], // row 9
/// ])));
///
/// assert_eq!(solutions.next(), Some(Board::from(&[
///     [9, 2, 6, 5, 7, 1, 4, 8, 3], // row 1
///     [3, 5, 1, 4, 8, 6, 2, 7, 9], // row 2
///     [8, 7, 4, 9, 2, 3, 5, 1, 6], // row 3
///     [5, 8, 2, 3, 6, 7, 1, 9, 4], // row 4
///     [1, 4, 9, 2, 5, 8, 3, 6, 7], // row 5
///     [7, 6, 3, 1, 9, 4, 8, 2, 5], // row 6
///     [2, 3, 8, 7, 4, 9, 6, 5, 1], // row 7
///     [6, 1, 7, 8, 3, 5, 9, 4, 2], // row 8
///     [4, 9, 5, 6, 1, 2, 7, 3, 8], // row 9
/// ])));
///
/// assert_eq!(solutions.next(), None);
/// # }
/// ```
#[derive(Debug)]
pub struct SolutionIter {
    first: bool,
    board: Board,
    stack: Vec<(usize, usize, BitIter<u16>)>,
}

impl SolutionIter {
    /// Construct a `SolutionIter` value from a [`Board`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::from(&[
    ///     [9, 0, 6, 0, 7, 0, 4, 0, 3], // row 1
    ///     [0, 0, 0, 4, 0, 0, 2, 0, 0], // row 2
    ///     [0, 7, 0, 0, 2, 3, 0, 1, 0], // row 3
    ///     [5, 0, 0, 0, 0, 0, 1, 0, 0], // row 4
    ///     [0, 4, 0, 2, 0, 8, 0, 6, 0], // row 5
    ///     [0, 0, 3, 0, 0, 0, 0, 0, 5], // row 6
    ///     [0, 3, 0, 7, 0, 0, 0, 5, 0], // row 7
    ///     [0, 0, 7, 0, 0, 5, 0, 0, 0], // row 8
    ///     [4, 0, 5, 0, 1, 0, 7, 0, 8], // row 9
    /// ]);
    ///
    /// let solutions = SolutionIter::new(&board);
    ///
    /// assert_eq!(solutions.count(), 2);
    /// # }
    /// ```
    pub fn new(board: &Board) -> Self {
        Self {
            first: true,
            board: *board,
            stack: Vec::with_capacity(BOARD_SIZE * BOARD_SIZE),
        }
    }
}

/// `From` implementation for `SolutionIter`.
impl From<Board> for SolutionIter {
    fn from(board: Board) -> Self {
        Self::new(&board)
    }
}

/// `Iterator` implementation for `SolutionIter`.
impl Iterator for SolutionIter {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;

            if valid(&self.board) {
                if let Some((x, y, values)) = cell_with_fewest_candidates(&self.board) {
                    if values == 0 {
                        return Some(self.board);
                    }

                    self.stack.push((x, y, values.into()));
                }
            }
        }

        if let Some((mut x, mut y, mut values)) = self.stack.pop() {
            loop {
                if let Some(value) = values.next() {
                    self.board.set_cell(x, y, value as u8);

                    if let Some(cs) = cell_with_fewest_candidates(&self.board) {
                        self.stack.push((x, y, values));

                        if cs.2 == 0 {
                            return Some(self.board);
                        }

                        x = cs.0;
                        y = cs.1;
                        values = cs.2.into();
                    }
                } else {
                    self.board.set_cell(x, y, 0);

                    if let Some(cs) = self.stack.pop() {
                        x = cs.0;
                        y = cs.1;
                        values = cs.2;
                    } else {
                        return None;
                    }
                }
            }
        } else {
            None
        }
    }
}

/// `FusedIterator` implementation for `SolutionIter`.
impl FusedIterator for SolutionIter {}
