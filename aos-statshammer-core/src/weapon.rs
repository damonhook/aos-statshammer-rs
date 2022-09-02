#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{abilities::weapon::*, DiceNotation, RollCharacteristic};

/// A `Weapon` struct represents a single weapon profile that belongs to an Age of Sigmar unit and
/// includes all of the profile characteristics for it.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weapon {
    pub models: u32,
    pub attacks: DiceNotation,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damage: DiceNotation,
    pub abilities: Vec<Ability>,
}

impl Weapon {
    /// Return a `Weapon` given the profile characteristics
    ///
    /// # Examples
    ///
    /// ```
    /// use aos_statshammer_core::{Weapon, DiceNotation};
    ///
    /// let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
    /// ```
    pub fn new(
        models: u32,
        attacks: DiceNotation,
        to_hit: u32,
        to_wound: u32,
        rend: u32,
        damage: DiceNotation,
        abilities: Vec<Ability>,
    ) -> Self {
        Self {
            models,
            attacks,
            to_hit,
            to_wound,
            rend,
            damage,
            abilities,
        }
    }

    pub fn reroll_ability(&self, phase: RollCharacteristic) -> Option<&Reroll> {
        self.abilities
            .iter()
            .filter_map(|ability| match ability {
                Ability::Reroll(a) if a.characteristic == phase => Some(a),
                _ => None,
            })
            .max_by_key(|a| a.reroll_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let output = Weapon::new(
            10,
            DiceNotation::from(2),
            3,
            4,
            1,
            DiceNotation::from(3),
            vec![],
        );
        let expected = Weapon {
            models: 10,
            attacks: DiceNotation::from(2),
            to_hit: 3,
            to_wound: 4,
            rend: 1,
            damage: DiceNotation::from(3),
            abilities: vec![],
        };
        assert_eq!(output, expected);
    }

    macro_rules! create_weapon {
        ($abilities: expr) => {
            Weapon {
                models: 10,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(3),
                abilities: $abilities,
            }
        };
    }

    #[test]
    fn reroll_ability_empty_items() {
        let weapon = create_weapon!(vec![]);
        assert_eq!(weapon.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_no_matching_charcteristic() {
        let weapon = create_weapon!(vec![Ability::from(Reroll {
            characteristic: RollCharacteristic::Wound,
            reroll_type: RerollType::Any
        })]);
        assert_eq!(weapon.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_reroll_found() {
        let weapon = create_weapon!(vec![
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Any
            }),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Any
            })
        );
    }

    #[test]
    fn reroll_ability_reroll_failed_found() {
        let weapon = create_weapon!(vec![
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones
            }),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed,
            })
        );
    }

    #[test]
    fn reroll_ability_reroll_ones_found() {
        let weapon = create_weapon!(vec![Ability::from(Reroll {
            characteristic: RollCharacteristic::Hit,
            reroll_type: RerollType::Ones
        })]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones,
            })
        );
    }
}
