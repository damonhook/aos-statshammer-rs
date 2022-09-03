#[doc(hidden)]
#[macro_export]
macro_rules! processor_results {
    ($r_all: expr) => {
        ProcessorResults::from([
            SaveResult::new(1, $r_all),
            SaveResult::new(2, $r_all),
            SaveResult::new(3, $r_all),
            SaveResult::new(4, $r_all),
            SaveResult::new(5, $r_all),
            SaveResult::new(6, $r_all),
            SaveResult::new(7, $r_all),
        ])
    };
    ($r1: expr, $r2: expr, $r3: expr, $r4: expr, $r5: expr, $r6: expr, $r7: expr) => {
        ProcessorResults::from([
            SaveResult::new(1, $r1),
            SaveResult::new(2, $r2),
            SaveResult::new(3, $r3),
            SaveResult::new(4, $r4),
            SaveResult::new(5, $r5),
            SaveResult::new(6, $r6),
            SaveResult::new(7, $r7),
        ])
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_processor_results_eq {
    ($left: expr, $right: expr) => {
        assert_processor_results_eq!($left, $right, 0.000_5);
    };
    ($left: expr, $right: expr, $precision: expr) => {
        assert!(matches!($left, ProcessorResults { .. }));
        assert_eq!($left.save_results.len(), $right.save_results.len());
        for (index, right_result) in $right.save_results.iter().enumerate() {
            assert_eq!($left.save_results[index].save, right_result.save);
            assert_float_eq!(
                $left.save_results[index].value,
                right_result.value,
                abs <= $precision
            );
        }
    };
}

pub mod weapons {
    use crate::{
        abilities::{weapon::*, RerollType},
        DiceNotation, RollCharacteristic as RollChar, Weapon,
    };

    pub mod gotrek {
        use super::*;

        pub fn zangrom_thaz() -> Weapon {
            Weapon {
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
            }
        }
    }

    pub mod hearthguard_berserkers {
        use super::*;

        pub fn broadaxes() -> Weapon {
            Weapon {
                models: 20,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 3,
                rend: 1,
                damage: DiceNotation::from(2),
                abilities: vec![Ability::from(LeaderExtraAttacks {
                    value: DiceNotation::from(1),
                    models: 1,
                })],
            }
        }
    }

    pub mod chainrasp_horde {
        use super::*;

        pub fn malignant_weapon() -> Weapon {
            Weapon {
                models: 10,
                attacks: 2.into(),
                to_hit: 4,
                to_wound: 4,
                rend: 0,
                damage: 1.into(),
                abilities: vec![Ability::from(LeaderExtraAttacks {
                    value: DiceNotation::from(1),
                    models: 1,
                })],
            }
        }
    }

    pub mod mortek_guard {
        use super::*;

        pub fn nadirite_blade() -> Weapon {
            Weapon {
                models: 9,
                attacks: 2.into(),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: 1.into(),
                abilities: vec![
                    Ability::from(LeaderExtraAttacks {
                        value: 1.into(),
                        models: 1,
                    }),
                    Ability::from(Exploding {
                        characteristic: RollChar::Hit,
                        on: 6,
                        unmodified: true,
                        extra: 1.into(),
                    }),
                ],
            }
        }

        pub fn soulcleaver_greatblade() -> Weapon {
            Weapon {
                models: 1,
                attacks: 2.into(),
                to_hit: 3,
                to_wound: 3,
                rend: 1,
                damage: 1.into(),
                abilities: vec![],
            }
        }
    }
}
