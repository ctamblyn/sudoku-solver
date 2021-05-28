//! # The `Board` type
//!
//! Puzzles and solutions are repesented by the `Board` type.
//!
//! A `Board` represents a nine-by-nine grid of cells.  Each cell contains either a number from 1
//! to 9, or a zero if the cell is unfilled.

mod tests;

/// The height or width of a "square" of cells within the board.  For standard sudoku puzzles, this
/// is 3.
pub const SQUARE_SIZE: usize = 3;

/// The number of cells in a row, column or square.  For standard sudoku puzzles, this is 3.
pub const BOARD_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

/// A representation of a puzzle or solution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    cells: [[u8; BOARD_SIZE]; BOARD_SIZE],
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
    /// let board = Board::from(&[[0u8; BOARD_SIZE]; BOARD_SIZE]);
    /// let board = board.with_cell(1, 1, 9);
    /// assert_eq!(board.get_cell(1, 1), 9);
    /// # }
    /// ```
    pub fn with_cell(&self, x: usize, y: usize, value: u8) -> Board {
        let mut b = *self;
        b.cells[y][x] = value;
        b
    }

    /// Test whether a board is complete.
    ///
    /// A board is regarded as complete if it contains no unfilled cells.
    ///
    /// Note that a compete board may not be a valid solution, since the game's constraints may not
    /// be satisfied.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # fn main() {
    /// # use sudoku_solver::*;
    /// let board = Board::from(&[
    ///     [0, 3, 5, 2, 6, 9, 7, 8, 1], // row 1
    ///     [6, 8, 2, 5, 7, 1, 4, 9, 3], // row 2
    ///     [1, 9, 7, 8, 3, 4, 5, 6, 2], // row 3
    ///     [8, 2, 6, 1, 9, 5, 3, 4, 7], // row 4
    ///     [3, 7, 4, 6, 8, 2, 9, 1, 5], // row 5
    ///     [9, 5, 1, 7, 4, 3, 6, 2, 8], // row 6
    ///     [5, 1, 9, 3, 2, 6, 8, 7, 4], // row 7
    ///     [2, 4, 8, 9, 5, 7, 1, 3, 6], // row 8
    ///     [7, 6, 3, 4, 1, 8, 2, 5, 9], // row 9
    /// ]);
    ///
    /// assert!(!board.complete());
    ///
    /// let board = board.with_cell(0, 0, 4);
    ///
    /// assert!(board.complete());
    /// # }
    /// ```
    pub fn complete(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| *cell != 0))
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
        Board { cells: *array_2d }
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
