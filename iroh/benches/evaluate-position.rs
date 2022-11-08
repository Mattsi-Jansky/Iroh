use criterion::{black_box, criterion_group, criterion_main, Criterion};

use iroh::search::search;
use iroh::state::GameState;
use iroh::game::Game;

fn benchmark(c: &mut Criterion) {
    let game = Game::from_fen("3k4/8/8/3p4/4P3/8/8/3K4 w - - 0 1");
    c.bench_function("evaluate", |b|
        b.iter(|| search(black_box(&game)))
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
