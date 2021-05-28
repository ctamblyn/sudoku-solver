//! # The `Board` type
//!
//! Puzzles and solutions are repesented by the `Board` type.
//!
//! A `Board` represents a nine-by-nine grid of cells.  Each cell contains either a number from 1
//! to 9, or a zero if the cell is unfilled.

/// The height or width of a "square" of cells within the board.  For standard sudoku puzzles,
/// this is 3.
pub const SQUARE_SIZE: usize = 3;

/// The number of cells in a row, column or square.  For standard sudoku puzzles, this is 3.
pub const BOARD_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

/// A representation of a puzzle or solution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    cells: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Create a `Board` with the given content.
    ///
    /// The `cells` parameter is a two dimensional array slice.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::new(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
    /// # println!("{}", board);
    /// # }
    /// ```
    pub fn new(cells: &[[u8; BOARD_SIZE]; BOARD_SIZE]) -> Board {
        Board { cells: *cells }
    }

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
    /// let board = Board::new(&[
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
    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.cells[y][x]
    }

    /// Construct a board which is obtained from the input board by modifying a single cell.
    ///
    /// The returned board is the same as the input one, except that the cell at column `x` and row
    /// `y` it will have the value `value`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::new(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
    /// let board = board.with_cell(1, 1, 9);
    /// assert_eq!(board.get_cell(1, 1), 9);
    /// # }
    /// ```
    pub fn with_cell(&self, x: usize, y: usize, value: u8) -> Board {
        let mut b = *self;
        b.cells[y][x] = value;
        b
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut y_prefix = "";

        for y in 0..BOARD_SIZE {
            write!(f, "{}", y_prefix)?;
            y_prefix = "\n";

            let mut x_prefix = "";
            for x in 0..BOARD_SIZE {
                write!(f, "{}", x_prefix)?;
                x_prefix = " ";

                let v = self.get_cell(x, y);
                if v != 0 {
                    write!(f, "{}", v)?;
                } else {
                    write!(f, "{}", '-')?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_rep_of_board_is_correct() {
        let board = Board::new(&[
            [1, 2, 3, 4, 5, 6, 7, 8, 9], // row 1
            [2, 3, 4, 5, 6, 7, 8, 9, 1], // row 2
            [3, 4, 5, 6, 7, 8, 9, 1, 2], // row 3
            [4, 5, 6, 7, 8, 9, 1, 2, 3], // row 4
            [5, 6, 7, 8, 9, 1, 2, 3, 4], // row 5
            [6, 7, 8, 9, 1, 2, 3, 4, 5], // row 6
            [7, 8, 9, 1, 2, 3, 4, 5, 6], // row 7
            [8, 9, 1, 2, 3, 4, 5, 6, 7], // row 8
            [9, 1, 2, 3, 4, 5, 6, 7, 8], // row 9
        ]);

        let str_rep = "1 2 3 4 5 6 7 8 9\n\
                       2 3 4 5 6 7 8 9 1\n\
                       3 4 5 6 7 8 9 1 2\n\
                       4 5 6 7 8 9 1 2 3\n\
                       5 6 7 8 9 1 2 3 4\n\
                       6 7 8 9 1 2 3 4 5\n\
                       7 8 9 1 2 3 4 5 6\n\
                       8 9 1 2 3 4 5 6 7\n\
                       9 1 2 3 4 5 6 7 8";

        assert_eq!(board.to_string(), str_rep);

        let board = Board::new(&[
            [1, 0, 0, 4, 0, 0, 7, 0, 0], // row 1
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 2
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 3
            [4, 0, 0, 7, 0, 0, 1, 0, 0], // row 4
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 5
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 6
            [7, 0, 0, 1, 0, 0, 4, 0, 0], // row 7
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 8
            [0, 0, 0, 0, 0, 0, 0, 0, 0], // row 9
        ]);

        let str_rep = "1 - - 4 - - 7 - -\n\
                       - - - - - - - - -\n\
                       - - - - - - - - -\n\
                       4 - - 7 - - 1 - -\n\
                       - - - - - - - - -\n\
                       - - - - - - - - -\n\
                       7 - - 1 - - 4 - -\n\
                       - - - - - - - - -\n\
                       - - - - - - - - -";

        assert_eq!(board.to_string(), str_rep);
    }
}
