mod characteristic;
pub use characteristic::{Characteristic, RollCharacteristic};

mod rollable;
pub use rollable::{Rollable, Dice, DiceNotation};

mod weapon;
pub use weapon::Weapon;

pub mod processors;
pub mod abilities;
