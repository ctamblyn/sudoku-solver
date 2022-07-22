use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku_solver::*;

fn hard_solvable_puzzle(c: &mut Criterion) {
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

    c.bench_function("hard solvable puzzle", |b| {
        b.iter(|| solve(black_box(&board)))
    });
}

fn hard_unsolvable_puzzle(c: &mut Criterion) {
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

    c.bench_function("hard unsolvable puzzle", |b| {
        b.iter(|| solve(black_box(&board)))
    });
}

fn large_solution_set(c: &mut Criterion) {
    let board = Board::from(&[
        [0, 2, 0, 0, 0, 0, 0, 0, 0], // row 1
        [0, 0, 0, 0, 0, 0, 0, 0, 3], // row 2
        [0, 7, 4, 0, 8, 0, 0, 0, 0], // row 3
        [0, 0, 0, 0, 0, 3, 0, 0, 2], // row 4
        [0, 8, 0, 0, 4, 0, 0, 1, 0], // row 5
        [6, 0, 0, 5, 0, 0, 0, 0, 0], // row 6
        [0, 0, 0, 0, 1, 0, 7, 8, 0], // row 7
        [5, 0, 0, 0, 0, 9, 0, 0, 0], // row 8
        [0, 0, 0, 0, 0, 0, 0, 4, 0], // row 9
    ]);

    c.bench_function("large solution set", |b| {
        b.iter(|| (black_box(SolutionIter::from(board).count())))
    });
}

criterion_group!(
    benches,
    hard_solvable_puzzle,
    hard_unsolvable_puzzle,
    large_solution_set
);

criterion_main!(benches);
