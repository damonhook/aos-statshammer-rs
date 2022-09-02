pub mod weapons {
    use crate::{abilities::weapon::*, DiceNotation, RollCharacteristic as RollChar, Weapon};

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
                    num_models: 1,
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
                    num_models: 1,
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
                        num_models: 1,
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
