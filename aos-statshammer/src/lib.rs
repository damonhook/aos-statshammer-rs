//! A crate for calculating and comparing damage outputs for Warhammer Age of Sigmar 
//! units (also referred to as Mathhammer).
//! 
//! # Features
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
//! # Examples
//! 
//! ```
//! use aos_statshammer::{
//!     abilities::{Ability, LeaderExtraAttacks, MortalWounds, Reroll},
//!     DiceNotation, RollCharacteristic, Unit, Weapon, UnitComparator,
//! };
//! 
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
//!             }),
//!             Ability::from(Reroll {
//!                 characteristic: RollCharacteristic::Wound,
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
//! let units = [gotrek, chainrasp_horde];
//! let output = UnitComparator::new(&units).compare_average_damage();
//! assert_eq!(output.len(), 2);
//! ```
//! 
//! *See [crate::AverageComparisonResult] for more info on the output structure of the above*
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


