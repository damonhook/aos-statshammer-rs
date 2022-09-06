use std::cmp;

use super::roll_target::{RollTarget, RollTargetValue};
use crate::{
    abilities::{opponent::OpponentAbility, weapon::Ability, RerollType},
    Characteristic, Dice, Opponent, RollCharacteristic as RollChar, Rollable,
    ValueCharacteristic as ValChar, Weapon,
};

// TODO:
// - Add docstrings and tests (need to move rand somewhere to allow mocking)

enum RerollPhase {
    Weapon(RollChar),
    Opponent,
}

/// A processor used for simulating the attack process for a given [Weapon].
/// See the [`simulate_damage`](Self::simulate_damage) for example usage
#[derive(Debug)]
pub struct SimulatedDamageProcessor<'a> {
    weapon: &'a Weapon,
    opponent: Option<&'a Opponent>,
    save: u32,
}

impl<'a> SimulatedDamageProcessor<'a> {
    pub fn new(weapon: &'a Weapon, save: u32) -> Self {
        Self {
            weapon,
            opponent: None,
            save,
        }
    }

    pub fn opponent(&mut self, opponent: &'a Opponent) -> &mut Self {
        self.opponent = Some(opponent);
        self
    }

    pub fn simulate_damage(&self) -> u32 {
        let mut total_attacks = self.weapon.models
            * (self.weapon.attacks.roll() + self.roll_bonus(ValChar::Attacks.into()));
        total_attacks += self
            .weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::LeaderExtraAttacks(a) => acc + a.models * a.value.roll(),
                _ => acc,
            });

        (0..total_attacks).fold(0, |acc, _| acc + self.simulate_attack())
    }

    fn simulate_attack(&self) -> u32 {
        let (hits, mut damage) = self.roll_phase(RollChar::Hit);
        if hits == 0 {
            return damage;
        }

        let (wounds, extra_wound_damage) = (0..hits).fold((0, 0), |acc, _| {
            let (wounds, damage) = self.roll_phase(RollChar::Wound);
            (acc.0 + wounds, acc.1 + damage)
        });
        damage += extra_wound_damage;
        if wounds == 0 {
            return damage;
        }

        let unsaved_wounds =
            (0..wounds).fold(0, |acc, _| if self.save_phase() { acc } else { acc + 1 });

        let damage_per_wound = self.weapon.damage.roll() + self.roll_bonus(ValChar::Damage.into());
        self.damage_with_ward(damage + (unsaved_wounds * damage_per_wound))
    }

    fn roll_phase(&self, phase: RollChar) -> (u32, u32) {
        let characteristic = match phase {
            RollChar::Hit => self.weapon.to_hit,
            RollChar::Wound => self.weapon.to_wound,
        };
        let mut target = RollTarget::new(characteristic, 0, Some(2));
        target += self.roll_bonus(phase.into()) as i32;
        let roll = self.roll_with_rerolls(RerollPhase::Weapon(phase), target);
        if roll >= target.modified() {
            let mut results = 1 + self.roll_exploding(phase, roll, target);
            let (mortal_wounds, in_addition) = self.roll_mortal_wounds(phase, roll, target);
            if !in_addition {
                results -= 1
            }
            (results, mortal_wounds)
        } else {
            (0, 0)
        }
    }

    fn save_phase(&self) -> bool {
        let mut target = RollTarget::new(self.save, 0, Some(cmp::max(2, self.save - 1)));
        if !self.opponent.map(|o| o.is_ethereal()).unwrap_or(false) {
            target -= (self.weapon.rend + self.roll_bonus(ValChar::Rend.into())) as i32;
            target += self.opponent.map_or(0, |o| {
                o.abilities.iter().fold(0, |acc, ability| match ability {
                    OpponentAbility::SaveBonus(a) => acc + a.value.roll() as i32,
                    _ => acc,
                })
            });
        }
        let roll = self.roll_with_rerolls(RerollPhase::Opponent, target);
        roll >= target.modified()
    }

    fn roll_bonus(&self, phase: Characteristic) -> u32 {
        self.weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Bonus(a) if a.characteristic == phase => acc + a.value.roll(),
                _ => acc,
            })
    }

    fn roll_with_rerolls(&self, phase: RerollPhase, target: RollTarget<u32, i32>) -> u32 {
        let roll = Dice::d6().roll();
        if roll < target.modified() {
            let reroll_type = match phase {
                RerollPhase::Weapon(p) => self.weapon.reroll_ability(p).map(|a| a.reroll_type),
                RerollPhase::Opponent => None,
            };
            let can_reroll = reroll_type.map_or(false, |r| match r {
                RerollType::Any => true,
                RerollType::Failed if roll < target.unmodified() as u32 => true,
                RerollType::Ones if roll == 1 => true,
                _ => false,
            });
            if can_reroll {
                return Dice::d6().roll();
            };
        }
        roll
    }

    fn roll_exploding(&self, phase: RollChar, roll: u32, target: RollTarget<u32, i32>) -> u32 {
        self.weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Exploding(a) if a.characteristic == phase => {
                    let exploding_target = target.clone_with_initial(a.on);
                    if roll >= exploding_target.value(a.unmodified) as u32 {
                        acc + a.extra.roll()
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    }

    fn roll_mortal_wounds(
        &self,
        phase: RollChar,
        roll: u32,
        target: RollTarget<u32, i32>,
    ) -> (u32, bool) {
        self.weapon
            .abilities
            .iter()
            .fold((0, true), |acc, ability| match ability {
                Ability::MortalWounds(a) if a.characteristic == phase => {
                    let mortal_target = target.clone_with_initial(a.on);
                    if roll >= mortal_target.value(a.unmodified) as u32 {
                        (acc.0 + a.mortals.roll(), acc.1 && a.in_addition)
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    }

    fn damage_with_ward(&self, damage: u32) -> u32 {
        let ward_saves = self.opponent.and_then(|opponent| {
            opponent.ward().map(|ward| {
                (0..damage).fold(0, |acc, _| {
                    if Dice::d6().roll() >= ward.on {
                        acc + 1
                    } else {
                        acc
                    }
                })
            })
        });
        damage - ward_saves.unwrap_or(0)
    }
}
