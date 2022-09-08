use super::roll_target::{RollTarget, RollTargetValue};
use crate::{
    abilities::{
        opponent::OpponentAbility, weapon::Ability, Characteristic, RerollType,
        RollCharacteristic as RollChar, ValueCharacteristic as ValChar,
    },
    rollable::Roller,
    Dice, Opponent, Rollable, Weapon,
};
use std::cmp;

// TODO:
// - Add docstrings and finish tests

enum RerollPhase {
    Weapon(RollChar),
    Opponent,
}

/// A processor used for simulating the attack process for a given [Weapon].
/// See the [`simulate_damage`](Self::simulate_damage) for example usage
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

    pub fn simulate_damage<T: Roller>(&self, roller: &T) -> u32 {
        let mut total_attacks = self.weapon.models
            * (self.weapon.attacks.roll(roller) + self.roll_bonus(ValChar::Attacks.into(), roller));
        total_attacks += self
            .weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::LeaderExtraAttacks(a) => acc + a.models * a.value.roll(roller),
                _ => acc,
            });

        (0..total_attacks).fold(0, |acc, _| acc + self.simulate_attack(roller))
    }

    fn simulate_attack<T: Roller>(&self, roller: &T) -> u32 {
        let (hits, mut damage) = self.roll_phase(RollChar::Hit, roller);
        if hits == 0 {
            return damage;
        }

        let (wounds, extra_wound_damage) = (0..hits).fold((0, 0), |acc, _| {
            let (wounds, damage) = self.roll_phase(RollChar::Wound, roller);
            (acc.0 + wounds, acc.1 + damage)
        });
        damage += extra_wound_damage;
        if wounds == 0 {
            return damage;
        }

        let unsaved_wounds = (0..wounds).fold(0, |acc, _| {
            if self.save_phase(roller) {
                acc
            } else {
                acc + 1
            }
        });

        let damage_per_wound =
            self.weapon.damage.roll(roller) + self.roll_bonus(ValChar::Damage.into(), roller);
        self.damage_with_ward(damage + (unsaved_wounds * damage_per_wound), roller)
    }

    fn roll_phase<T: Roller>(&self, phase: RollChar, roller: &T) -> (u32, u32) {
        let characteristic = match phase {
            RollChar::Hit => self.weapon.to_hit,
            RollChar::Wound => self.weapon.to_wound,
        };
        let mut target = RollTarget::new(characteristic, 0, Some(2));
        target += self.roll_bonus(phase.into(), roller) as i32;
        let roll = self.roll_with_rerolls(RerollPhase::Weapon(phase), target, roller);
        if roll >= target.modified() {
            let mut results = 1 + self.roll_exploding(phase, roll, target, roller);
            let (mortal_wounds, in_addition) = self.roll_mortal_wounds(phase, roll, target, roller);
            if !in_addition {
                results -= 1
            }
            (results, mortal_wounds)
        } else {
            (0, 0)
        }
    }

    fn save_phase<T: Roller>(&self, roller: &T) -> bool {
        let mut target = RollTarget::new(self.save, 0, Some(cmp::max(2, self.save - 1)));
        if !self.opponent.map(|o| o.is_ethereal()).unwrap_or(false) {
            target -= (self.weapon.rend + self.roll_bonus(ValChar::Rend.into(), roller)) as i32;
            target += self.opponent.map_or(0, |o| {
                o.abilities.iter().fold(0, |acc, ability| match ability {
                    OpponentAbility::SaveBonus(a) => acc + a.value.roll(roller) as i32,
                    _ => acc,
                })
            });
        }
        let roll = self.roll_with_rerolls(RerollPhase::Opponent, target, roller);
        roll >= target.modified()
    }

    fn roll_bonus<T: Roller>(&self, phase: Characteristic, roller: &T) -> u32 {
        self.weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Bonus(a) if a.characteristic == phase => acc + a.value.roll(roller),
                _ => acc,
            })
    }

    fn roll_with_rerolls<T: Roller>(
        &self,
        phase: RerollPhase,
        target: RollTarget<u32, i32>,
        roller: &T,
    ) -> u32 {
        let roll = Dice::d6().roll(roller);
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
                return Dice::d6().roll(roller);
            };
        }
        roll
    }

    fn roll_exploding<T: Roller>(
        &self,
        phase: RollChar,
        roll: u32,
        target: RollTarget<u32, i32>,
        roller: &T,
    ) -> u32 {
        self.weapon
            .abilities
            .iter()
            .fold(0, |acc, ability| match ability {
                Ability::Exploding(a) if a.characteristic == phase => {
                    let exploding_target = target.clone_with_initial(a.on);
                    if roll >= exploding_target.value(a.unmodified) as u32 {
                        acc + a.extra.roll(roller)
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    }

    fn roll_mortal_wounds<T: Roller>(
        &self,
        phase: RollChar,
        roll: u32,
        target: RollTarget<u32, i32>,
        roller: &T,
    ) -> (u32, bool) {
        self.weapon
            .abilities
            .iter()
            .fold((0, true), |acc, ability| match ability {
                Ability::MortalWounds(a) if a.characteristic == phase => {
                    let mortal_target = target.clone_with_initial(a.on);
                    if roll >= mortal_target.value(a.unmodified) as u32 {
                        let mortals = a.mortals.roll(roller);
                        (acc.0 + mortals, acc.1 && a.in_addition)
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
    }

    fn damage_with_ward<T: Roller>(&self, damage: u32, roller: &T) -> u32 {
        let ward_saves = self.opponent.and_then(|opponent| {
            opponent.ward().map(|ward| {
                (0..damage).fold(0, |acc, _| {
                    if Dice::d6().roll(roller) >= ward.on {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        abilities::{
            opponent::Ward,
            weapon::{Bonus, Exploding, MortalWounds},
        },
        rollable::MockRoller,
        DiceNotation,
    };
    use mockall::predicate::eq;
    use test_case::test_case;

    macro_rules! basic_weapon {
        () => {
            basic_weapon!(vec![])
        };
        ($abilities: expr) => {
            Weapon {
                models: 10,
                attacks: 2.into(),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: 2.into(),
                abilities: $abilities,
            }
        };
    }

    macro_rules! mock_rolls {
        ($roller: ident, $($roll: expr),+) => {
            $(
                $roller.expect_roll().times(1).return_const($roll);
            )*
        };
    }

    #[test]
    fn roll_bonus_no_abilities() {
        let weapon = basic_weapon!();
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let mock_roller = MockRoller::default();
        assert_eq!(
            processor.roll_bonus(ValChar::Attacks.into(), &mock_roller),
            0
        );
    }

    #[test]
    fn roll_bonus_with_abilities() {
        let weapon = basic_weapon!(vec![
            Ability::from(Bonus {
                characteristic: ValChar::Attacks.into(),
                value: 2.into()
            }),
            Ability::from(Bonus {
                characteristic: ValChar::Damage.into(),
                value: 2.into()
            }),
            Ability::from(Bonus {
                characteristic: ValChar::Attacks.into(),
                value: Dice::d(6).into()
            }),
        ]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let mut mock_roller = MockRoller::default();
        mock_roller
            .expect_roll()
            .with(eq(6))
            .times(1)
            .return_const(5u32);
        assert_eq!(
            processor.roll_bonus(ValChar::Attacks.into(), &mock_roller),
            7
        );
    }

    #[test]
    fn roll_exploding_no_abilities() {
        let weapon = basic_weapon!();
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 0, Some(2));
        let mock_roller = MockRoller::default();
        assert_eq!(
            processor.roll_exploding(RollChar::Hit, 6, target, &mock_roller),
            0
        );
    }

    #[test_case(3, true, 0 ; "unmodified with roll of 3")]
    #[test_case(5, true, 0 ; "unmodified with roll of 5")]
    #[test_case(6, true, 5 ; "unmodified with roll of 6")]
    #[test_case(3, false, 0 ; "modified with roll of 3")]
    #[test_case(5, false, 5 ; "modified with roll of 5")]
    #[test_case(6, false, 5 ; "modified with roll of 6")]
    fn roll_exploding_with_ability(roll: u32, unmodified: bool, expected: u32) {
        let weapon = basic_weapon!(vec![Ability::from(Exploding {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified,
            extra: DiceNotation::from(Dice::d(6)),
        })]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 1, Some(2));
        let mut mock_roller = MockRoller::default();
        mock_roller.expect_roll().return_const(5u32);
        let output = processor.roll_exploding(RollChar::Hit, roll, target, &mock_roller);
        assert_eq!(output, expected);
    }

    #[test]
    fn roll_exploding_multiple_abilities() {
        let weapon = basic_weapon!(vec![
            Ability::from(Exploding {
                characteristic: RollChar::Hit,
                on: 5,
                unmodified: true,
                extra: DiceNotation::from(1),
            }),
            Ability::from(Exploding {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                extra: DiceNotation::from(Dice::d(6)),
            }),
            Ability::from(Exploding {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: false,
                extra: DiceNotation::from(2),
            })
        ]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 1, Some(2));
        let output = processor.roll_exploding(RollChar::Hit, 5, target, &MockRoller::default());
        assert_eq!(output, 3);
    }

    #[test]
    fn roll_mortal_wounds_no_abilities() {
        let weapon = basic_weapon!();
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 0, Some(2));
        let mock_roller = MockRoller::default();
        assert_eq!(
            processor.roll_mortal_wounds(RollChar::Hit, 6, target, &mock_roller),
            (0, true)
        );
    }

    #[test_case(3, true, 0 ; "unmodified with roll of 3")]
    #[test_case(5, true, 0 ; "unmodified with roll of 5")]
    #[test_case(6, true, 5 ; "unmodified with roll of 6")]
    #[test_case(3, false, 0 ; "modified with roll of 3")]
    #[test_case(5, false, 5 ; "modified with roll of 5")]
    #[test_case(6, false, 5 ; "modified with roll of 6")]
    fn roll_mortal_wounds_output_damage(roll: u32, unmodified: bool, expected: u32) {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified,
            mortals: DiceNotation::from(Dice::d(6)),
            in_addition: false,
        })]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 1, Some(2));
        let mut mock_roller = MockRoller::default();
        mock_roller.expect_roll().return_const(5u32);
        let (output, _) = processor.roll_mortal_wounds(RollChar::Hit, roll, target, &mock_roller);
        assert_eq!(output, expected);
    }

    #[test_case(3, true, true ; "unsuccessful")]
    #[test_case(6, false, false ; "successful")]
    #[test_case(6, true, true ; "successful ability in addition")]
    fn roll_mortal_wounds_in_addition(roll: u32, in_addition: bool, expected: bool) {
        let weapon = basic_weapon!(vec![Ability::from(MortalWounds {
            characteristic: RollChar::Hit,
            on: 6,
            unmodified: true,
            mortals: DiceNotation::from(Dice::d(6)),
            in_addition,
        })]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 0, Some(2));
        let mut mock_roller = MockRoller::default();
        mock_roller.expect_roll().return_const(6u32);
        let (_, output) = processor.roll_mortal_wounds(RollChar::Hit, roll, target, &mock_roller);
        assert_eq!(output, expected);
    }

    #[test]
    fn roll_mortal_wounds_multiple_abilities() {
        let weapon = basic_weapon!(vec![
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 5,
                unmodified: true,
                mortals: DiceNotation::from(1),
                in_addition: false,
            }),
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: true,
                mortals: DiceNotation::from(Dice::d(6)),
                in_addition: true,
            }),
            Ability::from(MortalWounds {
                characteristic: RollChar::Hit,
                on: 6,
                unmodified: false,
                mortals: DiceNotation::from(2),
                in_addition: false,
            })
        ]);
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let target = RollTarget::new(3, 1, Some(2));
        let output = processor.roll_mortal_wounds(RollChar::Hit, 5, target, &MockRoller::default());
        assert_eq!(output, (3, false));
    }

    #[test]
    fn damage_with_ward_no_abilities() {
        let weapon = basic_weapon!();
        let processor = SimulatedDamageProcessor::new(&weapon, 4);
        let mock_roller = MockRoller::default();
        assert_eq!(processor.damage_with_ward(3, &mock_roller), 3);
    }

    #[test]
    fn damage_with_ward_with_ability_found() {
        let weapon = basic_weapon!();
        let mut processor = SimulatedDamageProcessor::new(&weapon, 4);
        let opponent = Opponent::new(vec![Ward { on: 5 }.into()]);
        processor.opponent(&opponent);

        let mut mock_roller = MockRoller::default();
        mock_rolls!(mock_roller, 4u32, 5u32, 6u32);

        assert_eq!(processor.damage_with_ward(3, &mock_roller), 1);
    }
}
