use super::*;

#[test]
fn default_board_is_empty() {
    let board = Board::default();

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            assert_eq!(board.get_cell(x, y), 0);
        }
    }
}

#[test]
fn set_cell_works() {
    let mut board = Board::default();

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            board.set_cell(x, y, 9);
            assert_eq!(board.get_cell(x, y), 9);

            board.set_cell(x, y, 0);
            assert_eq!(board.get_cell(x, y), 0);
        }
    }
}

#[test]
fn with_cell_works() {
    let board = Board::default();

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let new_board = board.with_cell(x, y, 9);
            assert_eq!(new_board.get_cell(x, y), 9);
            assert_eq!(board.get_cell(x, y), 0);
        }
    }
}

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
