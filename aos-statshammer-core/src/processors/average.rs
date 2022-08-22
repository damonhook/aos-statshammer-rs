use super::{roll_target::RollTarget, ProcessorResults};
use crate::{
    abilities::*,
    characteristic::{Characteristic as Char, RollCharacteristic as RollChar},
    rollable::Rollable,
    Weapon,
};

#[derive(Debug)]
pub struct AverageDamageProcessor<'a> {
    weapon: &'a Weapon,
    abilities: &'a AbilityManager,
}

impl<'a> AverageDamageProcessor<'a> {
    pub fn new(weapon: &'a Weapon, abilities: &'a AbilityManager) -> Self {
        Self { weapon, abilities }
    }

    pub fn average_damage(&self) -> ProcessorResults {
        let mut results = ProcessorResults::new();

        let attacks = self.average_attacks();
        let average_hits = self.roll_phase(attacks, RollChar::Hit, &mut results);
        let average_wounds = self.roll_phase(average_hits, RollChar::Wound, &mut results);

        for mut save_result in results.save_results.iter_mut() {
            save_result.value +=
                self.damage_phase(self.save_phase(average_wounds, save_result.save));
        }
        results
    }

    fn average_attacks(&self) -> f32 {
        let mut attacks_per_model = self.weapon.attacks.average();
        attacks_per_model += self.average_bonus(Char::Attacks);
        let mut attacks = (self.weapon.models as f32) * attacks_per_model;
        attacks += self.average_leader_extra_attacks();

        attacks
    }

    fn roll_phase(&self, base: f32, phase: RollChar, results: &mut ProcessorResults) -> f32 {
        let initial = match phase {
            RollChar::Hit => self.weapon.to_hit as f32,
            RollChar::Wound => self.weapon.to_wound as f32,
        };
        let mut target = RollTarget::new(initial, 0.0, Some(2.0));
        target += self.average_bonus(Char::Roll(phase));
        let base = base + self.average_rerolls(phase, base, target);
        let mut phase_result = base * self.roll_probability(target.modified());

        phase_result += self.average_exploding(phase, base, target);

        let (result_reduction, mortal_wounds) = self.mortal_wounds(phase, base, target);
        phase_result -= result_reduction;
        results.add_all(mortal_wounds);

        phase_result
    }

    fn save_phase(&self, wounds: f32, save: u32) -> f32 {
        let mut target = RollTarget::new(save as f32, 0.0, Some(2.0));
        target -= self.weapon.rend as f32;
        target -= self.average_bonus(Char::Rend);
        wounds * (self.inverse_roll_probability(target.modified()))
    }

    fn damage_phase(&self, wounds: f32) -> f32 {
        let mut damage_per_wound = self.weapon.damage.average();
        damage_per_wound += self.average_bonus(Char::Damage);
        wounds * damage_per_wound
    }

    fn roll_probability(&self, target: f32) -> f32 {
        if target > 7.0 {
            0.0
        } else {
            let numerator = 7.0 - target;
            (numerator / 6.0).max(0.0).min(1.0)
        }
    }

    fn inverse_roll_probability(&self, target: f32) -> f32 {
        1.0 - self.roll_probability(target)
    }

    fn average_bonus(&self, characteristic: Char) -> f32 {
        self.abilities.items.iter().fold(0.0, |acc, a| match a {
            Ability::Bonus(x) if x.characteristic == characteristic => acc + x.value.average(),
            _ => acc,
        })
    }

    fn average_leader_extra_attacks(&self) -> f32 {
        self.abilities.items.iter().fold(0.0, |acc, a| match a {
            Ability::LeaderExtraAttacks(x) => acc + ((x.num_models as f32) * x.value.average()),
            _ => acc,
        })
    }

    fn average_rerolls(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> f32 {
        match self.abilities.reroll_ability(phase) {
            Some(Ability::Reroll(_)) => base * self.inverse_roll_probability(target.modified()),
            Some(Ability::RerollFailed(_)) => {
                base * self.inverse_roll_probability(target.unmodified())
            }
            Some(Ability::RerollOnes(_)) => base / 6.0,
            _ => 0.0,
        }
    }

    fn average_exploding(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> f32 {
        self.abilities
            .items
            .iter()
            .find_map(|a| match a {
                Ability::Exploding(a) if a.characteristic == phase => {
                    let ability_target = RollTarget::new(a.on as f32, target.modifier, Some(2.0));
                    let ability_probability =
                        self.roll_probability(ability_target.value(a.unmodified));
                    Some(base * ability_probability * a.extra.average())
                }
                _ => None,
            })
            .unwrap_or(0.0)
    }

    fn mortal_wounds(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> (f32, f32) {
        self.abilities
            .items
            .iter()
            .find_map(|a| match a {
                Ability::MortalWounds(a) if a.characteristic == phase => {
                    let ability_target = RollTarget::new(a.on as f32, target.modifier, Some(2.0));
                    let num_mortals =
                        base * self.roll_probability(ability_target.value(a.unmodified));
                    let damage = num_mortals * a.mortals.average();
                    Some((if a.in_addition { 0.0 } else { num_mortals }, damage))
                }
                _ => None,
            })
            .unwrap_or((0.0, 0.0))
    }
}
