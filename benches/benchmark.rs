use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordsearch::*;

const WORDS: [&str; 19] = [
    "anxious",
    "blossom",
    "border",
    "coordinated",
    "follow",
    "guess",
    "hope",
    "impede",
    "initiate",
    "instrument",
    "mind",
    "nose",
    "plausible",
    "prescribe",
    "produce",
    "recite",
    "robin",
    "stress",
    "vivacious",
];

fn bench14x14(c: &mut Criterion) {
    let rows = 14;
    let cols = 14;

    c.bench_function("grid 14x14", |b| {
        b.iter(|| {
            generate_grid(black_box(rows), black_box(cols), black_box(&WORDS));
        })
    });
}

fn bench15x15(c: &mut Criterion) {
    let rows = 15;
    let cols = 15;

    c.bench_function("grid 15x15", |b| {
        b.iter(|| {
            generate_grid(black_box(rows), black_box(cols), black_box(&WORDS));
        })
    });
}

fn bench16x16(c: &mut Criterion) {
    let rows = 16;
    let cols = 16;

    c.bench_function("grid 16x16", |b| {
        b.iter(|| {
            generate_grid(black_box(rows), black_box(cols), black_box(&WORDS));
        })
    });
}

criterion_group!(benches, bench14x14, bench15x15, bench16x16);
criterion_main!(benches);
