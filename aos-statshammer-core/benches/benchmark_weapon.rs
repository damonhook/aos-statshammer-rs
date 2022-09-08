use aos_statshammer_core::{
    abilities::{weapon::*, RerollType, RollCharacteristic as RollChar},
    DiceNotation, Weapon,
};
use criterion::*;
use std::time::Duration;

fn benchmark_weapon_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Weapon Creation");
    group.bench_function("struct", |b| {
        b.iter(|| Weapon {
            models: 1,
            attacks: DiceNotation::from(6),
            to_hit: 3,
            to_wound: 3,
            rend: 2,
            damage: DiceNotation::from(3),
            abilities: vec![
                Ability::from(Reroll {
                    characteristic: RollChar::Hit,
                    reroll_type: RerollType::Any,
                }),
                Ability::from(Reroll {
                    characteristic: RollChar::Wound,
                    reroll_type: RerollType::Any,
                }),
                Ability::from(MortalWounds {
                    characteristic: RollChar::Hit,
                    on: 6,
                    unmodified: true,
                    mortals: DiceNotation::try_from("d6").unwrap(),
                    in_addition: true,
                }),
            ],
        })
    });
    group.bench_function("builder", |b| {
        b.iter(|| {
            Weapon::builder()
                .models(1)
                .attacks(6)
                .to_hit(3)
                .to_wound(3)
                .rend(2)
                .damage(3)
                .ability(Reroll {
                    characteristic: RollChar::Hit,
                    reroll_type: RerollType::Any,
                })
                .ability(Reroll {
                    characteristic: RollChar::Wound,
                    reroll_type: RerollType::Any,
                })
                .ability(MortalWounds {
                    characteristic: RollChar::Hit,
                    on: 6,
                    unmodified: true,
                    mortals: DiceNotation::try_from("d6").unwrap(),
                    in_addition: true,
                })
                .build()
                .unwrap()
        })
    });
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    benchmark_weapon_creation(c);
}

criterion_group! {
    name = benches;
    // These benchmarks generally run in <300ns so use a larger noise threshold
    config = Criterion::default().warm_up_time(Duration::from_millis(500)).noise_threshold(0.05);
    targets = bench
}
criterion_main!(benches);
