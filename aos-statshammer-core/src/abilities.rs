use serde::{Deserialize, Serialize};

use crate::DiceNotation;

use crate::{Characteristic, RollCharacteristic};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reroll {
    pub characteristic: RollCharacteristic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerollOnes {
    pub characteristic: RollCharacteristic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerollFailed {
    pub characteristic: RollCharacteristic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bonus {
    pub characteristic: Characteristic,
    pub value: DiceNotation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaderExtraAttacks {
    pub value: DiceNotation,
    pub num_models: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exploding {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub extra: DiceNotation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MortalWounds {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub mortals: DiceNotation,
    pub in_addition: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Ability {
    Reroll(Reroll),
    RerollFailed(RerollFailed),
    RerollOnes(RerollOnes),
    Bonus(Bonus),
    LeaderExtraAttacks(LeaderExtraAttacks),
    Exploding(Exploding),
    MortalWounds(MortalWounds),
}

macro_rules! enum_from_ability {
    ($struct_name:ident) => {
        impl From<$struct_name> for Ability {
            fn from(a: $struct_name) -> Self {
                Self::$struct_name(a)
            }
        }
    };
}

enum_from_ability!(Reroll);
enum_from_ability!(RerollFailed);
enum_from_ability!(RerollOnes);
enum_from_ability!(Bonus);
enum_from_ability!(LeaderExtraAttacks);
enum_from_ability!(Exploding);
enum_from_ability!(MortalWounds);
