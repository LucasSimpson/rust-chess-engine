use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use board::{Board};
use chess::board::{Board, ChessMove};
use chess::v2::{Manager, find_best_move};


fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("apply move", |ben| {
        let mut board: Board = Board::from_fen(["1r1qkbnr/2pnppBp/3p4/pp1P1P2/P6Q/8/1PPN1PPP/R3KB1R", "w", "-", "-", "100", "8"]).unwrap();
        let cm = ChessMove::from_long_algebraic_notation("d7f6");
        // time:   [141.96 us 142.29 us 142.65 us]
        ben.iter(|| {
            board.apply_move(&cm)
        });
    });

    // c.bench_function("iterate", |ben| {
    //     // time:
    //     ben.iter(|| {
    //         let mut manager = Manager::new();
    //         let mut board: Board = Board::from_fen(["1r1qkbnr/2pnppBp/3p4/pp1P1P2/P6Q/8/1PPN1PPP/R3KB1R", "w", "-", "-", "100", "8"]).unwrap();
    //         manager.find_best_move(board, 1);
    //     });
    // });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().measurement_time(Duration::new(3, 0)).sample_size(10);
    targets = criterion_benchmark,
}
criterion_main!(benches);
