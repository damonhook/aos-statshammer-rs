use criterion::*;

mod average;

pub fn bench(_c: &mut Criterion) {
    average::benches();
}

criterion_group!(benches, bench);
criterion_main!(benches);
