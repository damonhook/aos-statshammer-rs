use aos_statshammer::{abilities::*, DiceNotation, RollCharacteristic as RollChar, Unit, Weapon};

pub fn gotrek() -> Unit {
    Unit::new(
        "Gotrek",
        vec![Weapon {
            models: 1,
            attacks: DiceNotation::from(6),
            to_hit: 3,
            to_wound: 3,
            rend: 2,
            damage: DiceNotation::from(3),
            abilities: vec![
                Ability::from(Reroll {
                    characteristic: RollChar::Hit,
                }),
                Ability::from(Reroll {
                    characteristic: RollChar::Wound,
                }),
                Ability::from(MortalWounds {
                    characteristic: RollChar::Hit,
                    on: 6,
                    unmodified: true,
                    mortals: DiceNotation::try_from("d6").unwrap(),
                    in_addition: true,
                }),
            ],
        }],
    )
}

pub fn chainrasp_horde() -> Unit {
    Unit::new(
        "Chainrasp Horde",
        vec![Weapon {
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
        }],
    )
}
