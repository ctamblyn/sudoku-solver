#![cfg(test)]

use super::*;

#[test]
fn string_rep_of_board_is_correct() {
    let board = Board::from(&[
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

    let board = Board::from(&[
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
