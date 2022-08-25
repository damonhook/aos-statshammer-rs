use aos_statshammer_core::{
    abilities::*, processors::AverageDamageProcessor, Characteristic as Char, Dice, DiceNotation,
    RollCharacteristic as RollChar, Weapon,
};
use criterion::*;
use std::time::Duration;

mod inputs {
    use super::*;

    pub mod artificial {
        use super::*;

        pub fn no_abilities() -> (Weapon, AbilityManager) {
            let weapon = Weapon {
                models: 10,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(2),
            };
            (weapon, AbilityManager::empty())
        }

        pub fn only_rerolls() -> (Weapon, AbilityManager) {
            let weapon = Weapon {
                models: 10,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(2),
            };
            let abilities = vec![
                Ability::from(Reroll::new(RollChar::Hit)),
                Ability::from(RerollOnes::new(RollChar::Wound)),
            ];
            (weapon, AbilityManager::new(abilities))
        }

        pub fn large_mix() -> (Weapon, AbilityManager) {
            let weapon = Weapon {
                models: 10,
                attacks: DiceNotation::try_from("2d6").unwrap(),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::try_from("d3 + 1").unwrap(),
            };
            let abilities = vec![
                Ability::from(LeaderExtraAttacks::new(DiceNotation::from(1), 1)),
                Ability::from(Bonus::new(Char::Attacks, DiceNotation::from(Dice::d6()))),
                Ability::from(Bonus::new(Char::Roll(RollChar::Hit), DiceNotation::from(1))),
                Ability::from(Reroll::new(RollChar::Hit)),
                Ability::from(RerollOnes::new(RollChar::Wound)),
                Ability::from(Bonus::new(Char::Damage, DiceNotation::from(2))),
                Ability::from(Exploding::new(
                    RollChar::Hit,
                    6,
                    true,
                    DiceNotation::from(2),
                )),
                Ability::from(MortalWounds::new(
                    RollChar::Hit,
                    6,
                    false,
                    DiceNotation::from(6),
                    true,
                )),
            ];
            (weapon, AbilityManager::new(abilities))
        }
    }

    pub mod realistic {
        use super::*;

        pub fn gotrek_axe() -> (Weapon, AbilityManager) {
            let weapon = Weapon {
                models: 1,
                attacks: DiceNotation::from(6),
                to_hit: 3,
                to_wound: 3,
                rend: 2,
                damage: DiceNotation::from(3),
            };
            let abilities = vec![
                Ability::from(Reroll::new(RollChar::Hit)),
                Ability::from(Reroll::new(RollChar::Wound)),
                Ability::from(MortalWounds::new(
                    RollChar::Hit,
                    6,
                    true,
                    DiceNotation::try_from("d6").unwrap(),
                    true,
                )),
            ];
            (weapon, AbilityManager::new(abilities))
        }

        pub fn hearthguard_berserkers_broadaxes() -> (Weapon, AbilityManager) {
            let weapon = Weapon {
                models: 20,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 3,
                rend: 1,
                damage: DiceNotation::from(2),
            };
            let abilities = vec![Ability::from(LeaderExtraAttacks {
                value: DiceNotation::from(1),
                num_models: 1,
            })];
            (weapon, AbilityManager::new(abilities))
        }
    }
}

fn benchmark_average_damage_artificial(c: &mut Criterion) {
    let mut group = c.benchmark_group("Average Damage (Artificial)");
    let inputs = [
        ("Empty", inputs::artificial::no_abilities()),
        ("Just Rerolls", inputs::artificial::only_rerolls()),
        ("Large Mix", inputs::artificial::large_mix()),
    ];

    for (name, (weapon, abilities)) in inputs.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            abilities,
            |b, abilities| {
                b.iter(|| AverageDamageProcessor::new(&weapon, &abilities).average_damage())
            },
        );
    }
    group.finish();
}

fn benchmark_average_damage_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("Average Damage (Realistic)");
    let inputs = [
        ("Gotrek", inputs::realistic::gotrek_axe()),
        (
            "Hearthguard",
            inputs::realistic::hearthguard_berserkers_broadaxes(),
        ),
    ];

    for (name, (weapon, abilities)) in inputs.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            abilities,
            |b, abilities| {
                b.iter(|| AverageDamageProcessor::new(&weapon, &abilities).average_damage())
            },
        );
    }
    group.finish();
}

pub fn bench(c: &mut Criterion) {
    benchmark_average_damage_artificial(c);
    benchmark_average_damage_realistic(c);
}

criterion_group! {
    name = benches;
    // These benchmarks generally run in <300ns so use a larger noise threshold
    config = Criterion::default().warm_up_time(Duration::from_millis(500)).noise_threshold(0.05);
    targets = bench
}
criterion_main!(benches);
