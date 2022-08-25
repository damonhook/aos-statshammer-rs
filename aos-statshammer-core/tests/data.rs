use aos_statshammer_core::{abilities::*, DiceNotation, RollCharacteristic as RollChar, Weapon};

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

pub fn chainrasp_horde() -> (Weapon, AbilityManager) {
    let weapon = Weapon {
        models: 10,
        attacks: 2.into(),
        to_hit: 4,
        to_wound: 4,
        rend: 0,
        damage: 1.into(),
    };
    let abilities = vec![Ability::from(LeaderExtraAttacks {
        value: DiceNotation::from(1),
        num_models: 1,
    })];
    (weapon, AbilityManager::new(abilities))
}
