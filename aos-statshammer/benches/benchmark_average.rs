use aos_statshammer::{Unit, UnitComparator};
use criterion::*;
use std::time::Duration;

pub mod inputs {
    use super::*;
    use aos_statshammer_core::testutils::weapons;

    pub fn gotrek() -> Unit {
        Unit::new("Gotrek", vec![weapons::gotrek::zangrom_thaz()])
    }
}

fn benchmark_simulation(c: &mut Criterion) {
    c.bench_function("Simulated Comparison", |b| {
        b.iter(|| {
            let units = [inputs::gotrek()];
            let comparator = UnitComparator::new(&units, None);
            comparator.compare_simulated_damage(4, 1_000)
        })
    });
}

pub fn bench(c: &mut Criterion) {
    benchmark_simulation(c);
}

criterion_group! {
    name = benches;
    // These benchmarks generally run in <300ns so use a larger noise threshold
    config = Criterion::default().warm_up_time(Duration::from_millis(500)).noise_threshold(0.05);
    targets = bench
}
criterion_main!(benches);
