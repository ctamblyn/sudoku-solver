//! # The `Board` type
//!
//! Puzzles and solutions are repesented by the `Board` type.
//!
//! A `Board` represents a nine-by-nine grid of cells.  Each cell contains either a number from 1
//! to 9, or a zero if the cell is unfilled.

#[cfg(test)]
mod tests;

/// The height or width of a "square" of cells within the board.  For standard sudoku puzzles, this
/// is 3.
pub const SQUARE_SIZE: usize = 3;

/// The number of cells in a row, column or square.  For standard sudoku puzzles, this is 9.
pub const BOARD_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

/// A representation of a puzzle or solution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Board {
    cells: [[u16; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Get the contents of the cell at the given coordinates.
    ///
    /// This returns the contents of the cell at column `x` and row `y`.  A zero represents a cell
    /// which is unfilled, otherwise the value will be between 1 and 9 inclusive.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::from(&[
    ///     [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
    ///     [0, 0, 0, 6, 0, 0, 0, 0, 3], // row 2
    ///     [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
    ///     [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
    ///     [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
    ///     [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
    ///     [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
    ///     [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
    ///     [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
    /// ]);
    ///
    /// assert_eq!(board.get_cell(1, 0), 2);
    /// # }
    /// ```
    #[inline]
    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.get_cell_as_mask(x, y).trailing_zeros() as u8
    }

    #[doc(hidden)]
    #[inline]
    pub(crate) fn get_cell_as_mask(&self, x: usize, y: usize) -> u16 {
        self.cells[y][x]
    }

    #[doc(hidden)]
    #[inline]
    pub(crate) fn set_cell_as_mask(&mut self, x: usize, y: usize, value: u16) {
        self.cells[y][x] = value;
    }

    /// Set the contents of the cell at the given coordinates to the given value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let mut board = Board::from(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
    /// board.set_cell(1, 1, 9);
    /// assert_eq!(board.get_cell(1, 1), 9);
    /// # }
    /// ```
    #[inline]
    pub fn set_cell(&mut self, x: usize, y: usize, value: u8) {
        self.set_cell_as_mask(x, y, 1 << value);
    }
}

/// Construct a `Board` from a 2D array.
impl From<&[[u8; BOARD_SIZE]; BOARD_SIZE]> for Board {
    /// Create a `Board` with the given content.
    ///
    /// The `cells` parameter is a two dimensional array slice.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::from(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
    /// # println!("{}", board);
    /// # }
    /// ```
    fn from(array_2d: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        let mut board = Board::default();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                board.set_cell(x, y, array_2d[y][x]);
            }
        }
        board
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[0b000_000_000_1; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..BOARD_SIZE {
            if y != 0 {
                s.push('\n');
            }

            for x in 0..BOARD_SIZE {
                if x != 0 {
                    s.push(' ');
                }

                let v = self.get_cell(x, y);
                s.push(if v != 0 {
                    char::from_digit(v as u32, 10).unwrap()
                } else {
                    '-'
                });
            }
        }

        write!(f, "{}", s)
    }
}
