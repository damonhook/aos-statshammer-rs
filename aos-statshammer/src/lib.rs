#![doc(
    // TODO Bundle with build
    html_logo_url = "https://raw.githubusercontent.com/damonhook/aos-statshammer/master/docs/logo256.png"
)]
//! A crate for calculating and comparing damage outputs for Warhammer Age of Sigmar
//! units (also referred to as Mathhammer).
//!
//! # Key Functionality
//!
//! - Average Damage Statistics
//!     - Gather the average damage that each unit is expected to do against each possible save
//!     - This average is a calculated value (not simulated)
//! - Weapon Abilities
//!     - Add various abilities to your weapon profiles in order to replicate the vast
//!       array of unit rules.
//!     - Some examples are:
//!         - "Deal `x` Mortal Wounds on a roll of `y`"
//!         - "Reroll `x`"
//!         - "Unmodified rolls of `x`+ deal `y` mortal wounds"
//!
//! # Crate Features
//!
//! - `serde`: This adds serialisation and deserialisation functionality to many of the structs,
//! powered by the [`serde`] crate.
//!
//! # Examples
//!
//! ## Creating Units
//!
//! ```
//! use aos_statshammer::{abilities::weapon::*, DiceNotation, RollCharacteristic, Unit, Weapon};
//!
//! let chainrasp_horde = Unit::new(
//!     "Chainrasp Horde",
//!     vec![Weapon {
//!         models: 10,
//!         attacks: 2.into(),
//!         to_hit: 4,
//!         to_wound: 4,
//!         rend: 0,
//!         damage: 1.into(),
//!         abilities: vec![Ability::from(LeaderExtraAttacks {
//!             value: DiceNotation::from(1),
//!             num_models: 1,
//!         })],
//!     }],
//! );
//! ```
//!
//! ```
//! # use aos_statshammer::{abilities::weapon::*, DiceNotation, RollCharacteristic, Unit, Weapon};
//! #
//! let gotrek = Unit::new(
//!     "Gotrek",
//!     vec![Weapon {
//!         models: 1,
//!         attacks: DiceNotation::from(6),
//!         to_hit: 3,
//!         to_wound: 3,
//!         rend: 2,
//!         damage: DiceNotation::from(3),
//!         abilities: vec![
//!             Ability::from(Reroll {
//!                 characteristic: RollCharacteristic::Hit,
//!                 reroll_type: RerollType::Any,
//!             }),
//!             Ability::from(Reroll {
//!                 characteristic: RollCharacteristic::Wound,
//!                 reroll_type: RerollType::Any,
//!             }),
//!             Ability::from(MortalWounds {
//!                 characteristic: RollCharacteristic::Hit,
//!                 on: 6,
//!                 unmodified: true,
//!                 mortals: DiceNotation::try_from("d6").unwrap(),
//!                 in_addition: true,
//!             }),
//!         ],
//!     }],
//! );
//! ```
//!
//! ## Compare Average and Maximum Damage values
//!
//! Let us take the `gotrek` and `chainrasp_horde` units we made eariler and do a comparison.
//!
//! ```
//! # use aos_statshammer::{abilities::weapon::*, DiceNotation, RollCharacteristic, Unit, Weapon};
//! use aos_statshammer::UnitComparator;
//!
//! # let chainrasp_horde = Unit::new(
//! #     "Chainrasp Horde",
//! #     vec![Weapon {
//! #         models: 10,
//! #         attacks: 2.into(),
//! #         to_hit: 4,
//! #         to_wound: 4,
//! #         rend: 0,
//! #         damage: 1.into(),
//! #         abilities: vec![Ability::from(LeaderExtraAttacks {
//! #             value: DiceNotation::from(1),
//! #             num_models: 1,
//! #         })],
//! #     }],
//! # );
//! # let gotrek = Unit::new(
//! #     "Gotrek",
//! #     vec![Weapon {
//! #         models: 1,
//! #         attacks: DiceNotation::from(6),
//! #         to_hit: 3,
//! #         to_wound: 3,
//! #         rend: 2,
//! #         damage: DiceNotation::from(3),
//! #         abilities: vec![
//! #             Ability::from(Reroll {
//! #                 characteristic: RollCharacteristic::Hit,
//! #                 reroll_type: RerollType::Any,
//! #             }),
//! #             Ability::from(Reroll {
//! #                 characteristic: RollCharacteristic::Wound,
//! #                 reroll_type: RerollType::Any,
//! #             }),
//! #             Ability::from(MortalWounds {
//! #                 characteristic: RollCharacteristic::Hit,
//! #                 on: 6,
//! #                 unmodified: true,
//! #                 mortals: DiceNotation::try_from("d6").unwrap(),
//! #                 in_addition: true,
//! #             }),
//! #         ],
//! #     }],
//! # );
//! let units = [chainrasp_horde, gotrek];
//! let output = UnitComparator::new(&units).compare_average_damage();
//! # assert_eq!(output.len(), 2);
//! ```
//!
//! *See [`AverageComparisonResult`](crate::AverageComparisonResult) for more info on the
//! output structure of the above*
//!
//! # Notes
//! The core logic for calculating the damage values is done by the
//! [aos-statshammer-core](aos_statshammer_core) crate (which is included as a dependency).

mod unit;
pub use unit::Unit;

// Re-export components needed for this lib
pub use aos_statshammer_core::{
    Characteristic, DiceNotation, RollCharacteristic, Rollable, Weapon,
};

mod compare;
pub use compare::{AverageComparisonResult, UnitComparator};

pub mod abilities;
