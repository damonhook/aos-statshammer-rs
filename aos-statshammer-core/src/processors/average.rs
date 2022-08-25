use super::{roll_target::RollTarget, ProcessorResults};
use crate::{
    abilities::*,
    characteristic::{Characteristic as Char, RollCharacteristic as RollChar},
    Rollable, Weapon,
};

// TODO:
// - Collapse the separate iter().folds() used for each roll based ability into a single loop
// - Roll leader extra attacks into average bonus

#[derive(Debug)]
pub struct AverageDamageProcessor<'a> {
    weapon: &'a Weapon,
}

impl<'a> AverageDamageProcessor<'a> {
    pub fn new(weapon: &'a Weapon) -> Self {
        Self { weapon }
    }

    /// Calculate the average damage for each save value for the given `weapon` and `abilities`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aos_statshammer_core::processors::AverageDamageProcessor;
    /// use aos_statshammer_core::{Weapon, DiceNotation};
    ///
    /// // Create a `Weapon`
    /// let weapon = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(2), vec![]);
    /// // Create a processor with borrowing the `Weapon` you created
    /// let processor = AverageDamageProcessor::new(&weapon);
    /// // Call the function to calculate the average damage
    /// let results = processor.average_damage();
    /// ```
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
        let mut phase_result = base * roll_probability(target.modified());

        phase_result += self.average_exploding(phase, base, target);

        let (mortal_wounds, result_reduction) = self.mortal_wounds(phase, base, target);
        phase_result -= result_reduction;
        results.add_all(mortal_wounds);

        phase_result
    }

    fn save_phase(&self, wounds: f32, save: u32) -> f32 {
        let mut target = RollTarget::new(save as f32, 0.0, Some(2.0));
        target -= self.weapon.rend as f32;
        target -= self.average_bonus(Char::Rend);
        wounds * (inverse_roll_probability(target.modified()))
    }

    fn damage_phase(&self, wounds: f32) -> f32 {
        let mut damage_per_wound = self.weapon.damage.average();
        damage_per_wound += self.average_bonus(Char::Damage);
        wounds * damage_per_wound
    }

    /// Get the average bonus to a given `characteristic` based on the `Bonus` abilities present
    /// for said `characteristic`
    fn average_bonus(&self, characteristic: Char) -> f32 {
        self.weapon.abilities.iter().fold(0.0, |acc, a| match a {
            Ability::Bonus(x) if x.characteristic == characteristic => acc + x.value.average(),
            _ => acc,
        })
    }

    /// Get the average number of extra attacks resulting from `LeaderExtraAttacks` abilities
    fn average_leader_extra_attacks(&self) -> f32 {
        self.weapon.abilities.iter().fold(0.0, |acc, a| match a {
            Ability::LeaderExtraAttacks(x) => acc + ((x.num_models as f32) * x.value.average()),
            _ => acc,
        })
    }

    /// Get the average number of rerolls for a given `characteristic`
    fn average_rerolls(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> f32 {
        match self.weapon.reroll_ability(phase) {
            Some(Ability::Reroll(_)) => base * inverse_roll_probability(target.modified()),
            Some(Ability::RerollFailed(_)) => base * inverse_roll_probability(target.unmodified()),
            Some(Ability::RerollOnes(_)) => base / 6.0,
            _ => 0.0,
        }
    }

    /// Get the average extra value resulting from any `Exploding` ability found for the
    /// given `characteristic`.
    fn average_exploding(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> f32 {
        self.weapon.abilities.iter().fold(0.0, |acc, a| match a {
            Ability::Exploding(a) if a.characteristic == phase => {
                let ability_target = RollTarget::new(a.on as f32, target.modifier, Some(2.0));
                let ability_probability = roll_probability(ability_target.value(a.unmodified));
                acc + base * ability_probability * a.extra.average()
            }
            _ => acc,
        })
    }

    /// Get the average damage that resulting from any `MortalWounds` abilities for the given
    /// `characteristic`, it also returns the base reduction if applicable.
    /// Returned tuple is in order of `(damage, base_reduction)`.
    fn mortal_wounds(&self, phase: RollChar, base: f32, target: RollTarget<f32>) -> (f32, f32) {
        self.weapon
            .abilities
            .iter()
            .fold((0.0, 0.0), |acc, a| match a {
                Ability::MortalWounds(a) if a.characteristic == phase => {
                    let ability_target = RollTarget::new(a.on as f32, target.modifier, Some(2.0));
                    let num_mortals = base * roll_probability(ability_target.value(a.unmodified));
                    let damage = num_mortals * a.mortals.average();
                    let base_reduction = if a.in_addition { 0.0 } else { num_mortals };
                    (acc.0 + damage, acc.1.max(base_reduction))
                }
                _ => acc,
            })
    }
}

fn roll_probability(target: f32) -> f32 {
    if target > 7.0 {
        0.0
    } else {
        ((7.0 - target) / 6.0).clamp(0.0, 1.0)
    }
}

fn inverse_roll_probability(target: f32) -> f32 {
    1.0 - roll_probability(target)
}

// ======================================
//               UNIT TESTS
// ======================================

#[cfg(test)]
mod tests {
    use crate::{processors::SaveResult, DiceNotation};

    use super::*;
    use float_eq::assert_float_eq;
    use test_case::test_case;

    static PRECISION: f32 = 0.000_5; // Approximately 3 decimal places

    #[test_case(1.0, 1.0    ; "0+")]
    #[test_case(1.0, 1.0    ; "1+")]
    #[test_case(2.0, 0.833  ; "2+")]
    #[test_case(3.0, 0.667  ; "3+")]
    #[test_case(4.0, 0.5    ; "4+")]
    #[test_case(5.0, 0.333  ; "5+")]
    #[test_case(6.0, 0.167  ; "6+")]
    #[test_case(7.0, 0.0    ; "7+")]
    fn roll_probability_for_target(target: f32, expected: f32) {
        let output = roll_probability(target);
        assert_float_eq!(output, expected, abs <= 0.0005);
    }

    #[test_case(1.0, 0.0    ; "0-")]
    #[test_case(1.0, 0.0    ; "1-")]
    #[test_case(2.0, 0.167  ; "2-")]
    #[test_case(3.0, 0.333  ; "3-")]
    #[test_case(4.0, 0.5    ; "4-")]
    #[test_case(5.0, 0.667  ; "5-")]
    #[test_case(6.0, 0.833  ; "6-")]
    #[test_case(7.0, 1.0    ; "7-")]
    fn inverse_roll_probability_for_target(target: f32, expected: f32) {
        let output = inverse_roll_probability(target);
        assert_float_eq!(output, expected, abs <= PRECISION);
    }

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

    macro_rules! assert_processor_results_eq {
        ($left: expr, $right: expr) => {
            assert!(matches!($left, ProcessorResults { .. }));
            assert_eq!($left.save_results.len(), $right.save_results.len());
            for (index, right_result) in $right.save_results.iter().enumerate() {
                assert_eq!($left.save_results[index].save, right_result.save);
                assert_float_eq!(
                    $left.save_results[index].value,
                    right_result.value,
                    abs <= PRECISION
                );
            }
        };
    }

    macro_rules! processor_results {
        ($r_all: expr) => {
            ProcessorResults::from([
                SaveResult::new(1, $r_all),
                SaveResult::new(2, $r_all),
                SaveResult::new(3, $r_all),
                SaveResult::new(4, $r_all),
                SaveResult::new(5, $r_all),
                SaveResult::new(6, $r_all),
                SaveResult::new(7, $r_all),
            ])
        };
        ($r1: expr, $r2: expr, $r3: expr, $r4: expr, $r5: expr, $r6: expr, $r7: expr) => {
            ProcessorResults::from([
                SaveResult::new(1, $r1),
                SaveResult::new(2, $r2),
                SaveResult::new(3, $r3),
                SaveResult::new(4, $r4),
                SaveResult::new(5, $r5),
                SaveResult::new(6, $r6),
                SaveResult::new(7, $r7),
            ])
        };
    }

    #[test]
    fn average_attacks_no_ability() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(processor.average_attacks(), 20.0, abs <= PRECISION);
    }

    #[test]
    fn average_attacks_with_abilities() {
        let weapon = basic_weapon!(vec![
            Ability::from(Bonus {
                characteristic: Char::Attacks,
                value: 1.into(),
            }),
            Ability::from(LeaderExtraAttacks {
                value: 1.into(),
                num_models: 1,
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(processor.average_attacks(), 31.0, abs <= PRECISION);
    }

    #[test_case(RollChar::Hit, 1.333 ; "hit")]
    #[test_case(RollChar::Wound, 1.0 ; "wound")]
    fn roll_phase_no_abilities(phase: RollChar, expected: f32) {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        let mut results = ProcessorResults::new();
        assert_float_eq!(
            processor.roll_phase(2.0, phase, &mut results),
            expected,
            abs <= PRECISION
        );
        assert_processor_results_eq!(results, ProcessorResults::new());
    }

    #[test_case(RollChar::Hit, 1.667 ; "hit")]
    #[test_case(RollChar::Wound, 1.333 ; "wound")]
    fn roll_phase_with_bonus_abilities(phase: RollChar, expected: f32) {
        let weapon = basic_weapon!(vec![
            Ability::from(Bonus {
                characteristic: Char::Roll(RollChar::Hit),
                value: 2.into(),
            }),
            Ability::from(Bonus {
                characteristic: Char::Roll(RollChar::Wound),
                value: 1.into(),
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        let mut results = ProcessorResults::new();
        assert_float_eq!(
            processor.roll_phase(2.0, phase, &mut results),
            expected,
            abs <= PRECISION
        );
        assert_processor_results_eq!(results, ProcessorResults::new());
    }

    #[test]
    fn roll_phase_with_exploding_abilities() {
        let weapon = basic_weapon!(vec![Ability::from(Exploding {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            extra: 2.into(),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        let mut results = ProcessorResults::new();
        assert_float_eq!(
            processor.roll_phase(2.0, RollChar::Hit, &mut results),
            2.0,
            abs <= PRECISION
        );
        assert_processor_results_eq!(results, ProcessorResults::new());
    }

    #[test]
    fn roll_phase_with_mortal_wounds_abilities() {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            mortals: 2.into(),
            in_addition: false,
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        let mut results = ProcessorResults::new();
        assert_float_eq!(
            processor.roll_phase(2.0, RollChar::Hit, &mut results),
            1.0,
            abs <= PRECISION
        );
        assert_processor_results_eq!(results, processor_results!(0.667));
    }

    #[test]
    fn save_phase_no_abilities() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(processor.save_phase(10.0, 3), 5.0, abs <= PRECISION);
    }

    #[test]
    fn save_phase_with_abilities() {
        let weapon = basic_weapon!(vec![Ability::from(Bonus {
            characteristic: Char::Rend,
            value: 1.into(),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(processor.save_phase(10.0, 3), 6.667, abs <= PRECISION);
    }

    #[test]
    fn damage_phase_no_abilities() {
        let weapon = basic_weapon!(vec![Ability::from(Bonus {
            characteristic: Char::Rend,
            value: 1.into(),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.damage_phase(2.0), 4.0);
    }

    #[test]
    fn damage_phase_with_abilities() {
        let weapon = basic_weapon!(vec![Ability::from(Bonus {
            characteristic: Char::Damage,
            value: DiceNotation::try_from("d6").unwrap(),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.damage_phase(2.0), 11.0);
    }

    #[test]
    fn average_bonus_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_bonus(Char::Attacks), 0.0);
    }

    #[test]
    fn average_bonus_single_ability_found() {
        let weapon = basic_weapon!(vec![Ability::from(Bonus {
            characteristic: Char::Attacks,
            value: DiceNotation::from(2),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_bonus(Char::Attacks), 2.0);
    }

    #[test]
    fn average_bonus_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(Bonus {
                characteristic: Char::Attacks,
                value: DiceNotation::from(2),
            }),
            Ability::from(Bonus {
                characteristic: Char::Attacks,
                value: DiceNotation::try_from("d6").unwrap(),
            }),
            Ability::from(Bonus {
                characteristic: Char::Damage, // Test that it correctly filters by characteristic
                value: DiceNotation::from(2),
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_bonus(Char::Attacks), 5.5);
    }

    #[test]
    fn average_leader_extra_attacks_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_leader_extra_attacks(), 0.0);
    }

    #[test]
    fn average_leader_extra_attacks_single_ability_found() {
        let weapon = basic_weapon!(vec![Ability::from(LeaderExtraAttacks {
            value: DiceNotation::from(2),
            num_models: 1,
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_leader_extra_attacks(), 2.0);
    }

    #[test]
    fn average_leader_extra_attacks_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(LeaderExtraAttacks {
                value: DiceNotation::from(2),
                num_models: 1,
            }),
            Ability::from(LeaderExtraAttacks {
                value: DiceNotation::try_from("d6").unwrap(),
                num_models: 2,
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(processor.average_leader_extra_attacks(), 9.0);
    }

    #[test]
    fn exploding_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(
            processor.average_exploding(RollChar::Hit, 2.0, RollTarget::from(3.0)),
            0.0
        );
    }

    #[test_case(true, 0.667 ; "unmodified")]
    #[test_case(false, 1.333 ; "modified")]
    fn exploding_single_ability_found(unmodified: bool, expected: f32) {
        let weapon = basic_weapon!(vec![Ability::from(Exploding {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified,
            extra: DiceNotation::from(2),
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(
            processor.average_exploding(RollChar::Hit, 2.0, RollTarget::new(3.0, 1.0, None)),
            expected,
            abs <= PRECISION
        );
    }

    #[test]
    fn exploding_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(Exploding {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                extra: DiceNotation::from(2),
            }),
            Ability::from(Exploding {
                characteristic: RollChar::Hit,
                on: 5,
                unmodified: true,
                extra: DiceNotation::try_from("d6").unwrap(),
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        assert_float_eq!(
            processor.average_exploding(RollChar::Hit, 2.0, RollTarget::new(3.0, 1.0, None)),
            3.0,
            abs <= PRECISION
        );
    }

    #[test]
    fn mortal_wounds_no_ability_found() {
        let weapon = basic_weapon!();
        let processor = AverageDamageProcessor::new(&weapon);
        assert_eq!(
            processor.mortal_wounds(RollChar::Hit, 2.0, RollTarget::from(3.0)),
            (0.0, 0.0)
        );
    }

    #[test_case(false, false, (1.333, 0.667) ; "modified")]
    #[test_case(true, false, (0.667, 0.333) ; "unmodified")]
    #[test_case(false, true, (1.333, 0.0) ; "modified in addition")]
    #[test_case(true, true, (0.667, 0.0) ; "unmodified in addition")]
    fn mortal_wounds_single_ability_found(
        unmodified: bool,
        in_addition: bool,
        expected: (f32, f32),
    ) {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified,
            mortals: DiceNotation::from(2),
            in_addition,
        })]);
        let processor = AverageDamageProcessor::new(&weapon);
        let (damage, base_reduction) =
            processor.mortal_wounds(RollChar::Hit, 2.0, RollTarget::new(3.0, 1.0, None));
        assert_float_eq!(damage, expected.0, abs <= PRECISION);
        assert_float_eq!(base_reduction, expected.1, abs <= PRECISION);
    }

    #[test]
    fn mortal_wounds_multiple_abilities_found() {
        let weapon = basic_weapon!(vec![
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                mortals: DiceNotation::from(2),
                in_addition: false,
            }),
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 5,
                unmodified: true,
                mortals: DiceNotation::try_from("d6").unwrap(),
                in_addition: false,
            }),
        ]);
        let processor = AverageDamageProcessor::new(&weapon);
        let (damage, base_reduction) =
            processor.mortal_wounds(RollChar::Hit, 2.0, RollTarget::new(3.0, 1.0, None));
        assert_float_eq!(damage, 3.0, abs <= PRECISION);
        assert_float_eq!(base_reduction, 0.667, abs <= PRECISION);
    }
}
