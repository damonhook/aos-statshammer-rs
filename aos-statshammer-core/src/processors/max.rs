use std::cmp;

use crate::{
    abilities::*, RollCharacteristic as RollChar, Rollable, ValueCharacteristic as VChar, Weapon,
};

// TODO:
// - Roll leader extra attacks into max bonus

/// A processor used for calculating the maximum damage for a given [Weapon].
/// See the [`max_damage`](Self::max_damage) for example usage
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
            * cmp::max(
                self.weapon.attacks.max() + self.max_bonus(VChar::Attacks),
                0,
            );
        attacks += self.max_leader_extra_attacks();

        let hit_rolls = attacks + self.max_exploding(RollChar::Hit, attacks);
        let (hit_mortals, hit_mortals_in_addition) =
            self.max_mortal_wounds(RollChar::Hit, hit_rolls);

        let wound_rolls = hit_rolls + self.max_exploding(RollChar::Wound, hit_rolls);
        let (wound_mortals, wound_mortals_in_addition) =
            self.max_mortal_wounds(RollChar::Wound, wound_rolls);

        let damage_per_wound =
            cmp::max(self.weapon.damage.max() + self.max_bonus(VChar::Damage), 0);
        let damage =
            (wound_rolls * damage_per_wound) + hit_mortals_in_addition + wound_mortals_in_addition;
        cmp::max(damage, cmp::max(hit_mortals, wound_mortals))
    }

    fn max_bonus(&self, characteristic: VChar) -> u32 {
        self.weapon.abilities.iter().fold(0, |acc, a| match a {
            Ability::Bonus(x) if x.characteristic == characteristic.into() => acc + x.value.max(),
            _ => acc,
        })
    }

    fn max_leader_extra_attacks(&self) -> u32 {
        self.weapon.abilities.iter().fold(0, |acc, a| match a {
            Ability::LeaderExtraAttacks(x) => acc + (x.num_models * x.value.max()),
            _ => acc,
        })
    }

    fn max_exploding(&self, phase: RollChar, current: u32) -> u32 {
        let total = self
            .weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Exploding(a) if a.characteristic == phase => acc + a.extra.max(),
                _ => acc,
            });
        current * cmp::max(total, 0)
    }

    fn max_mortal_wounds(&self, phase: RollChar, current: u32) -> (u32, u32) {
        let (total_mortals, mortals_in_addition) =
            self.weapon
                .abilities
                .iter()
                .fold((0, 0), |acc, ability| match ability {
                    Ability::MortalWounds(a) if a.characteristic == phase => {
                        let max_mortals = a.mortals.max();
                        if a.in_addition {
                            (acc.0 + max_mortals, acc.1 + max_mortals)
                        } else {
                            (acc.0 + max_mortals, acc.1)
                        }
                    }
                    _ => acc,
                });
        (current * total_mortals, current * mortals_in_addition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiceNotation;
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

    #[test]
    fn max_leader_extra_attacks_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_leader_extra_attacks(), 0);
    }

    #[test]
    fn max_leader_extra_attacks_single_ability_found() {
        let weapon = basic_weapon!(vec![Ability::from(LeaderExtraAttacks {
            value: DiceNotation::try_from("d6").unwrap(),
            num_models: 2,
        })]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_leader_extra_attacks(), 12);
    }

    #[test]
    fn max_leader_extra_attacks_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(LeaderExtraAttacks {
                value: DiceNotation::try_from("d6").unwrap(),
                num_models: 2,
            }),
            Ability::from(LeaderExtraAttacks {
                value: DiceNotation::from(2),
                num_models: 1,
            })
        ]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_leader_extra_attacks(), 14);
    }

    #[test]
    fn max_bonus_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_bonus(VChar::Attacks.into()), 0);
    }

    #[test]
    fn max_bonus_single_ability_found() {
        let weapon = basic_weapon!(vec![Ability::from(Bonus {
            characteristic: VChar::Attacks.into(),
            value: DiceNotation::try_from("d6").unwrap(),
        })]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_bonus(VChar::Attacks.into()), 6);
    }

    #[test]
    fn max_bonus_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(Bonus {
                characteristic: VChar::Attacks.into(),
                value: DiceNotation::try_from("d6").unwrap(),
            }),
            Ability::from(Bonus {
                characteristic: VChar::Attacks.into(),
                value: DiceNotation::from(2),
            })
        ]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_bonus(VChar::Attacks.into()), 8);
    }

    #[test]
    fn max_exploding_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_exploding(RollChar::Hit, 1), 0);
    }

    #[test]
    fn max_exploding_single_ability_found() {
        let weapon = basic_weapon!(vec![Ability::from(Exploding {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            extra: DiceNotation::try_from("d6").unwrap(),
        })]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_exploding(RollChar::Hit, 1), 6);
    }

    #[test]
    fn max_mortal_wounds_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_mortal_wounds(RollChar::Hit, 4), (0, 0));
    }

    #[test_case(true, (24, 24); "in addition")]
    #[test_case(false, (24, 0) ; "not in addition")]
    fn max_mortal_wounds_single_ability_found(in_addition: bool, expected: (u32, u32)) {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            mortals: DiceNotation::try_from("d6").unwrap(),
            in_addition,
        })]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_mortal_wounds(RollChar::Hit, 4), expected);
    }

    #[test]
    fn max_mortal_wounds_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                mortals: DiceNotation::try_from("d6").unwrap(),
                in_addition: false,
            }),
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                mortals: DiceNotation::from(2),
                in_addition: false,
            }),
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: false,
                mortals: DiceNotation::from(2),
                in_addition: true,
            })
        ]);
        let processor = MaxDamageProcessor::new(&weapon);
        assert_eq!(processor.max_mortal_wounds(RollChar::Hit, 4), (40, 8));
    }
}
