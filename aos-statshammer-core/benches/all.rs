use criterion::*;

mod benchmark_average;
mod benchmark_weapon;

pub fn bench(_c: &mut Criterion) {
    benchmark_average::benches();
    benchmark_weapon::benches();
}

criterion_group!(benches, bench);
criterion_main!(benches);
