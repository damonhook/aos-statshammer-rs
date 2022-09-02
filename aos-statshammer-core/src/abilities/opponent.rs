use super::common::create_abilities_enum;
use crate::DiceNotation;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use super::common::RerollType;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SaveReroll {
    pub reroll_type: RerollType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SaveBonus {
    pub value: DiceNotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ward {
    pub on: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ethereal {}

create_abilities_enum! {
    enum_name = OpponentAbility,
    abilities = [
        SaveReroll,
        Ward,
        SaveBonus,
        Ethereal
    ],
    used_for_doclink = "[`Opponent`](crate::Opponent)"
}
