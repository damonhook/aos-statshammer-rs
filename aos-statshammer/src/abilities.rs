pub use aos_statshammer_core::abilities::{
    Ability, Bonus, Exploding, LeaderExtraAttacks, MortalWounds, Reroll, RerollFailed, RerollOnes,
}; // Re-export abilities from core lib

mod definitions;
pub use definitions::AbilityDefinition;
pub mod fields;
