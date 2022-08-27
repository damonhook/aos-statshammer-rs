use std::cmp;

use crate::{abilities::*, Characteristic as Char, Rollable, Weapon};

// TODO:
// - Collapse the separate iter().folds() used for each roll based ability into a single loop
// - Roll leader extra attacks into max bonus

#[derive(Debug)]
pub struct MaxDamageProcessor<'a> {
    weapon: &'a Weapon,
}

impl<'a> MaxDamageProcessor<'a> {
    pub fn new(weapon: &'a Weapon) -> Self {
        Self { weapon }
    }


    /// Calculate the maximum damage for the given `weapon`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aos_statshammer_core::processors::MaxDamageProcessor;
    /// use aos_statshammer_core::{Weapon, DiceNotation};
    ///
    /// // Create a `Weapon`
    /// let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
    /// // Create a processor with borrowing the `Weapon` you created
    /// let processor = MaxDamageProcessor::new(&weapon);
    /// // Call the function to calculate the maximum damage
    /// let results = processor.max_damage();
    /// ```
    pub fn max_damage(&self) -> u32 {
        let mut attacks = self.weapon.models
            * cmp::max(self.weapon.attacks.max() + self.max_bonus(Char::Attacks), 0);
        attacks += self.max_leader_extra_attakcs();
        let rolls = attacks + self.max_exploding();

        let mut damage_per_wound =
            cmp::max(self.weapon.damage.max() + self.max_bonus(Char::Damage), 0);
        damage_per_wound += self.max_mortal_wounds(damage_per_wound);
        rolls * damage_per_wound
    }

    fn max_bonus(&self, characteristic: Char) -> u32 {
        self.weapon.abilities.iter().fold(0, |acc, a| match a {
            Ability::Bonus(x) if x.characteristic == characteristic => acc + x.value.max(),
            _ => acc,
        })
    }

    fn max_leader_extra_attakcs(&self) -> u32 {
        self.weapon.abilities.iter().fold(0, |acc, a| match a {
            Ability::LeaderExtraAttacks(x) => acc + x.value.max(),
            _ => acc,
        })
    }

    fn max_exploding(&self) -> u32 {
        let total = self
            .weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Exploding(a) => acc + (acc * a.extra.max()),
                _ => acc,
            });
        cmp::max(total, 0)
    }

    fn max_mortal_wounds(&self, current: u32) -> u32 {
        self.weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::MortalWounds(a) => {
                    let max_mortals = a.mortals.max();
                    if a.in_addition || max_mortals > current {
                        acc + a.mortals.max()
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DiceNotation, RollCharacteristic as RollChar};
    use test_case::test_case;

    macro_rules! basic_weapon {
        () => {
            basic_weapon!(vec![])
        };
        ($abilities: expr) => {
            Weapon {
                models: 10,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(2),
                abilities: $abilities,
            }
        };
    }

    #[test_case(4, false, 6 ; "lower than current(4)")]
    #[test_case(12, false, 0 ; "higher than current(12)")]
    #[test_case(4, true, 6; "in addition to current(4)")]
    #[test_case(12, true, 6; "in addition to current(12)")]
    fn max_mortal_wounds_single_ability_found(current: u32, in_addition: bool, expected: u32) {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            mortals: DiceNotation::try_from("d6").unwrap(),
            in_addition,
        })]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_mortal_wounds(current), expected);
    }
}
