use std::fs::File;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordplay_core::{
    dictionary::{DictSearch, Dictionary},
    normalized_word::NormalizedWord,
};

fn enable_bench(c: &mut Criterion) {
    let enable = Dictionary::from_file(File::open("data/enable.txt").unwrap());

    c.bench_function("enable find banana", |b| {
        let banana = NormalizedWord::from_str_safe("banana");
        b.iter(|| enable.find(black_box(&banana)))
    });

    c.bench_function("enable search bana??", |b| {
        b.iter(|| {
            enable
                .iter_search(black_box(DictSearch::from_pattern("bana??")))
                .count()
        })
    });

    c.bench_function("enable search ban prefix", |b| {
        b.iter(|| {
            enable
                .iter_search(black_box(DictSearch::from_pattern("ban")))
                .count()
        })
    });

    c.bench_function("enable search ?an prefix", |b| {
        b.iter(|| {
            enable
                .iter_search(black_box(DictSearch::from_pattern("?an")))
                .count()
        })
    });
}

criterion_group!(benches, enable_bench);
criterion_main!(benches);
