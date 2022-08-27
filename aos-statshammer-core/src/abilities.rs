use crate::DiceNotation;

use crate::{Characteristic, RollCharacteristic};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reroll {
    pub characteristic: RollCharacteristic,
}
impl Reroll {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerollOnes {
    pub characteristic: RollCharacteristic,
}
impl RerollOnes {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerollFailed {
    pub characteristic: RollCharacteristic,
}
impl RerollFailed {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bonus {
    pub characteristic: Characteristic,
    pub value: DiceNotation,
}
impl Bonus {
    pub fn new(characteristic: Characteristic, value: DiceNotation) -> Self {
        Self {
            characteristic,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderExtraAttacks {
    pub value: DiceNotation,
    pub num_models: u32,
}
impl LeaderExtraAttacks {
    pub fn new(value: DiceNotation, num_models: u32) -> Self {
        Self { value, num_models }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exploding {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub extra: DiceNotation,
}
impl Exploding {
    #[allow(unused)]
    pub fn new(
        characteristic: RollCharacteristic,
        on: u32,
        unmodified: bool,
        extra: DiceNotation,
    ) -> Self {
        Self {
            characteristic,
            on,
            unmodified,
            extra,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MortalWounds {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub mortals: DiceNotation,
    pub in_addition: bool,
}
impl MortalWounds {
    #[allow(unused)]
    pub fn new(
        characteristic: RollCharacteristic,
        on: u32,
        unmodified: bool,
        mortals: DiceNotation,
        in_addition: bool,
    ) -> Self {
        Self {
            characteristic,
            on,
            unmodified,
            mortals,
            in_addition,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
