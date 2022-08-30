use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum representing the different characteristics that a `Weapon` can roll for
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum RollCharacteristic {
    Hit,
    Wound,
}

impl fmt::Display for RollCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hit => write!(f, "Hit"),
            Self::Wound => write!(f, "Wound"),
        }
    }
}

/// Enum representing the different characteristics that a `Weapon` can have a value for
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ValueCharacteristic {
    Attacks,
    Rend,
    Damage,
}

impl fmt::Display for ValueCharacteristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attacks => write!(f, "Attacks"),
            Self::Rend => write!(f, "Rend"),
            Self::Damage => write!(f, "Damage"),
        }
    }
}

/// Enum representing the different characteristics that a `Weapon` can have.
/// This is made up of [RollCharacteristic] and [ValueCharacteristic].
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Characteristic {
    Value(ValueCharacteristic),
    Roll(RollCharacteristic),
}

impl From<RollCharacteristic> for Characteristic {
    fn from(roll: RollCharacteristic) -> Self {
        Self::Roll(roll)
    }
}

impl From<ValueCharacteristic> for Characteristic {
    fn from(val: ValueCharacteristic) -> Self {
        Self::Value(val)
    }
}

impl fmt::Display for Characteristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(c) => write!(f, "{}", c),
            Self::Roll(c) => write!(f, "{}", c),
        }
    }
}
