use criterion::{criterion_group, criterion_main, Criterion};
use shortid_rs::unique_short_code;
use uuid::Uuid;
use std::collections::HashSet;

fn bench_generate(c: &mut Criterion) {
    let mut existing = HashSet::new();
    c.bench_function("generate_short_code", |b| {
        b.iter(|| {
            let uuid = Uuid::new_v4();
            unique_short_code(&uuid, &mut existing);
        })
    });
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
