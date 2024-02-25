use criterion::{black_box, criterion_group, criterion_main, Criterion};

use iroh::game::Game;
use iroh::search::search;

fn search_basic(c: &mut Criterion) {
    let mut game = Game::from_fen("3k4/8/8/3p4/4P3/8/8/3K4 w - - 0 1");
    c.bench_function("search_basic", |b| b.iter(|| search(black_box(&mut game))));
}

fn search_complex(c: &mut Criterion) {
    let mut game = Game::from_fen("r3kb1r/ppq2ppp/2p5/3pN3/3P4/8/PPPQ1PPP/R3R1K1 w kq - 0 1");
    c.bench_function("search_complex", |b| {
        b.iter(|| search(black_box(&mut game)))
    });
}

criterion_group!(benches, search_basic, search_complex);
criterion_main!(benches);
