use super::{create_abilities_enum, Characteristic, RerollType, RollCharacteristic};
use crate::DiceNotation;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reroll {
    pub characteristic: RollCharacteristic,
    pub reroll_type: RerollType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bonus {
    pub characteristic: Characteristic,
    pub value: DiceNotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeaderExtraAttacks {
    pub value: DiceNotation,
    pub models: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Exploding {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub extra: DiceNotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MortalWounds {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub mortals: DiceNotation,
    pub in_addition: bool,
}

create_abilities_enum! {
    enum_name = Ability,
    abilities = [
        Reroll,
        Bonus,
        LeaderExtraAttacks,
        Exploding,
        MortalWounds
    ],
    used_for_doclink = "[`Weapon`](crate::Weapon)"
}
