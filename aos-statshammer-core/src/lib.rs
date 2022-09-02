#![doc(
    // TODO Bundle with build
    html_logo_url = "https://raw.githubusercontent.com/damonhook/aos-statshammer/master/docs/logo256.png"
)]
//! This crate contains the core logic for calculating damage outputs for a singular
//! Warhammer Age of Sigmar weapon profile.
//!
//! # Crate Features
//!
//! - `serde`: This adds serialisation and deserialisation functionality to many of the structs,
//! powered by the [`serde`] crate.
//!
//! # Examples
//!
//! ## Creating a `Weapon`
//!
//! ```
//! use aos_statshammer_core::{Weapon, DiceNotation};
//!
//! let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
//! ```
//!
//! For more details see the documentation for [`Weapon`].
//!
//! ## Calculating average damage for a `Weapon`
//!
//! Let us take one of the [`Weapons`](Weapon) that we previously created and then use the
//! [`AverageDamageProcessor`](processors::AverageDamageProcessor) to caluclate the average damage
//! for each save value
//!
//! ```
//! use aos_statshammer_core::processors::AverageDamageProcessor;
//! # use aos_statshammer_core::{Weapon, DiceNotation};
//!
//! # let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
//! let results = AverageDamageProcessor::new(&weapon).average_damage();
//! ```
//!
//! ## Calculating maximum damage for a `Weapon`
//!
//! Let us take one of the [`Weapons`](Weapon) that we previously created and then use the
//! [`MaxDamageProcessor`](processors::MaxDamageProcessor) to caluclate the maximum damage
//!
//! ```
//! use aos_statshammer_core::processors::MaxDamageProcessor;
//! # use aos_statshammer_core::{Weapon, DiceNotation};
//!
//! # let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
//! let results = MaxDamageProcessor::new(&weapon).max_damage();
//! ```

mod characteristic;
pub use characteristic::{Characteristic, RollCharacteristic, ValueCharacteristic};

mod rollable;
pub use rollable::{Dice, DiceNotation, Rollable};

mod weapon;
pub use weapon::{Weapon, WeaponBuilder};

mod opponent;
pub use opponent::Opponent;

pub mod abilities;

pub mod processors;

#[doc(hidden)]
pub mod testutils;
