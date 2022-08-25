use std::cmp;

use crate::{abilities::*, Characteristic as Char, Rollable, Weapon};

#[derive(Debug)]
pub struct MaxDamageProcessor<'a> {
    weapon: &'a Weapon,
    abilities: &'a AbilityManager,
}

impl<'a> MaxDamageProcessor<'a> {
    pub fn new(weapon: &'a Weapon, abilities: &'a AbilityManager) -> Self {
        Self { weapon, abilities }
    }

    pub fn max_damage(&self) -> u32 {
        let attacks = cmp::max(self.weapon.attacks.max() + self.max_bonus(Char::Attacks), 0);
        let rolls = attacks + self.max_exploding(attacks);
        let mut damage_per_wound =
            cmp::max(self.weapon.damage.max() + self.max_bonus(Char::Damage), 0);
        damage_per_wound = self.max_damage_with_mortal_wounds(damage_per_wound);
        self.weapon.models * rolls * damage_per_wound
    }

    fn max_bonus(&self, characteristic: Char) -> u32 {
        self.abilities.items.iter().fold(0, |acc, a| match a {
            Ability::Bonus(x) if x.characteristic == characteristic => acc + x.value.max(),
            _ => acc,
        })
    }

    fn max_exploding(&self, base: u32) -> u32 {
        let total = self
            .abilities
            .items
            .iter()
            .fold(base, |acc, ability| match ability {
                Ability::Exploding(a) => acc + (acc * a.extra.max()),
                _ => acc,
            });
        cmp::max(total, 0)
    }

    fn max_damage_with_mortal_wounds(&self, current: u32) -> u32 {
        self.abilities
            .items
            .iter()
            .fold(current, |acc, ability| match ability {
                Ability::MortalWounds(a) => {
                    if a.in_addition {
                        acc + a.mortals.max()
                    } else {
                        cmp::max(acc + a.mortals.max(), acc)
                    }
                }
                _ => acc,
            })
    }
}
