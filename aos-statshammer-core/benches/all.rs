use criterion::*;

mod benchmark_average;

pub fn bench(_c: &mut Criterion) {
    benchmark_average::benches();
}

criterion_group!(benches, bench);
criterion_main!(benches);
