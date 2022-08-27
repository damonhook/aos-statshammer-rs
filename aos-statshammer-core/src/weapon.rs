use crate::{abilities::*, DiceNotation, RollCharacteristic};

/// A `Weapon` struct represents a single weapon profile that belongs to an Age of Sigmar unit and
/// includes all of the profile characteristics for it.
#[derive(Debug, PartialEq, Eq)]
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

    pub fn reroll_ability(&self, phase: RollCharacteristic) -> Option<&Ability> {
        let find_reroll = || {
            self.abilities.iter().find_map(|ability| match ability {
                ab @ Ability::Reroll(x) if x.characteristic == phase => Some(ab),
                _ => None,
            })
        };
        let find_reroll_failed = || {
            self.abilities.iter().find_map(|ability| match ability {
                ab @ Ability::RerollFailed(x) if x.characteristic == phase => Some(ab),
                _ => None,
            })
        };
        let find_reroll_ones = || {
            self.abilities.iter().find_map(|ability| match ability {
                ab @ Ability::RerollOnes(x) if x.characteristic == phase => Some(ab),
                _ => None,
            })
        };

        find_reroll().or_else(|| find_reroll_failed().or_else(find_reroll_ones))
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
        let weapon = create_weapon!(vec![Ability::from(Reroll::new(RollCharacteristic::Wound))]);
        assert_eq!(weapon.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_reroll_found() {
        let weapon = create_weapon!(vec![
            Ability::from(RerollFailed::new(RollCharacteristic::Hit)),
            Ability::from(RerollOnes::new(RollCharacteristic::Hit)),
            Ability::from(Reroll::new(RollCharacteristic::Hit)),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(Reroll::new(RollCharacteristic::Hit)))
        );
    }

    #[test]
    fn reroll_ability_reroll_failed_found() {
        let weapon = create_weapon!(vec![
            Ability::from(RerollFailed::new(RollCharacteristic::Hit)),
            Ability::from(RerollOnes::new(RollCharacteristic::Hit)),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(RerollFailed::new(RollCharacteristic::Hit)))
        );
    }

    #[test]
    fn reroll_ability_reroll_ones_found() {
        let weapon = create_weapon!(vec![Ability::from(RerollOnes::new(
            RollCharacteristic::Hit
        ))]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(RerollOnes::new(RollCharacteristic::Hit)))
        );
    }
}
