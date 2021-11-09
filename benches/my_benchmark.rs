use criterion::{criterion_group, criterion_main, Criterion};
use guarding_ident::ModelBuilder;
use std::path::PathBuf;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lambda", |b| b.iter(|| {
        ModelBuilder::build_models_by_dir(PathBuf::from("."));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);