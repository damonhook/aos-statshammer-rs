mod unit;
pub use unit::Unit;

// Re-export components needed for this lib
pub use aos_statshammer_core::{
    abilities, Characteristic, DiceNotation, RollCharacteristic, Rollable, Weapon,
};

mod compare;
pub use compare::{AverageComparisonResult, UnitComparator};
