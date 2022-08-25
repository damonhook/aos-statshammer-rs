mod characteristic;
pub use characteristic::{Characteristic, RollCharacteristic};

mod rollable;
pub use rollable::{Dice, DiceNotation, Rollable};

mod weapon;
pub use weapon::Weapon;

pub mod abilities;
pub mod processors;
