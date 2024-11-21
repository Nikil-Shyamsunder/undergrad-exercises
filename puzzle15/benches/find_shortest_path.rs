use criterion::{criterion_group, criterion_main, Criterion};
// use std::hint::black_box;
use puzzle15::{find_shortest_path, find_shortest_path_inefficient, GameState, Move}; // Replace `your_crate` with your crate name


// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);

fn benchmark_shortest_path(c: &mut Criterion) {
    let test_cases = vec![
        vec![Move::TopToBottom, Move::TopToBottom, Move::TopToBottom],
        vec![Move::TopToBottom, Move::LeftToRight, Move::BottomToTop],
        vec![
            Move::TopToBottom,
            Move::LeftToRight,
            Move::LeftToRight,
            Move::LeftToRight,
            Move::TopToBottom,
        ],
    ];

    for (i, expected_moves) in test_cases.iter().enumerate() {
        let mut state = GameState::default();
        assert_eq!(state.perform_moves(expected_moves), expected_moves.len());

        c.bench_function(&format!("find_shortest_path - Test Case {}", i + 1), |b| {
            b.iter(|| {
                let result = find_shortest_path(GameState::default(), state.clone());
                assert_eq!(result, *expected_moves);
            });
        });

        c.bench_function(
            &format!("find_shortest_path_inefficient - Test Case {}", i + 1),
            |b| {
                b.iter(|| {
                    let result = find_shortest_path_inefficient(GameState::default(), state.clone());
                    assert_eq!(result, *expected_moves);
                });
            },
        );
    }
}

criterion_group!(benches, benchmark_shortest_path);
criterion_main!(benches);
