use crate::rollable::DiceNotation;

use crate::characteristic::{Characteristic, RollCharacteristic};

#[derive(Debug, Clone, PartialEq)]
pub struct Reroll {
    pub characteristic: RollCharacteristic,
}
impl Reroll {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RerollOnes {
    pub characteristic: RollCharacteristic,
}
impl RerollOnes {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RerollFailed {
    pub characteristic: RollCharacteristic,
}
impl RerollFailed {
    pub fn new(characteristic: RollCharacteristic) -> Self {
        Self { characteristic }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bonus {
    pub characteristic: Characteristic,
    pub value: DiceNotation,
}
impl Bonus {
    pub fn new(characteristic: Characteristic, value: DiceNotation) -> Self {
        Self {
            characteristic,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeaderExtraAttacks {
    pub value: DiceNotation,
    pub num_models: u32,
}
impl LeaderExtraAttacks {
    pub fn new(value: DiceNotation, num_models: u32) -> Self {
        Self { value, num_models }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exploding {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub extra: DiceNotation,
}
impl Exploding {
    #[allow(unused)]
    pub fn new(
        characteristic: RollCharacteristic,
        on: u32,
        unmodified: bool,
        extra: DiceNotation,
    ) -> Self {
        Self {
            characteristic,
            on,
            unmodified,
            extra,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MortalWounds {
    pub characteristic: RollCharacteristic,
    pub on: u32,
    pub unmodified: bool,
    pub mortals: DiceNotation,
    pub in_addition: bool,
}
impl MortalWounds {
    #[allow(unused)]
    pub fn new(
        characteristic: RollCharacteristic,
        on: u32,
        unmodified: bool,
        mortals: DiceNotation,
        in_addition: bool,
    ) -> Self {
        Self {
            characteristic,
            on,
            unmodified,
            mortals,
            in_addition,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ability {
    Reroll(Reroll),
    RerollFailed(RerollFailed),
    RerollOnes(RerollOnes),
    Bonus(Bonus),
    LeaderExtraAttacks(LeaderExtraAttacks),
    Exploding(Exploding),
    MortalWounds(MortalWounds),
}

macro_rules! enum_from_ability {
    ($struct_name:ident) => {
        impl From<$struct_name> for Ability {
            fn from(a: $struct_name) -> Self {
                Self::$struct_name(a)
            }
        }
    };
}

enum_from_ability!(Reroll);
enum_from_ability!(RerollFailed);
enum_from_ability!(RerollOnes);
enum_from_ability!(Bonus);
enum_from_ability!(LeaderExtraAttacks);
enum_from_ability!(Exploding);
enum_from_ability!(MortalWounds);

#[derive(Debug)]
pub struct AbilityManager {
    pub items: Vec<Ability>,
}
impl AbilityManager {
    pub fn new(items: Vec<Ability>) -> Self {
        Self { items }
    }

    pub fn reroll_ability(&self, phase: RollCharacteristic) -> Option<&Ability> {
        let find_reroll = || {
            self.items.iter().find_map(|ability| match ability {
                ab @ Ability::Reroll(x) if x.characteristic == phase => Some(ab),
                _ => None,
            })
        };
        let find_reroll_failed = || {
            self.items.iter().find_map(|ability| match ability {
                ab @ Ability::RerollFailed(x) if x.characteristic == phase => Some(ab),
                _ => None,
            })
        };
        let find_reroll_ones = || {
            self.items.iter().find_map(|ability| match ability {
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
    fn reroll_ability_empty_items() {
        let abilities = AbilityManager { items: vec![] };
        assert_eq!(abilities.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_no_matching_charcteristic() {
        let abilities = AbilityManager {
            items: vec![Ability::from(Reroll::new(RollCharacteristic::Wound))],
        };
        assert_eq!(abilities.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_reroll_found() {
        let abilities = AbilityManager {
            items: vec![
                Ability::from(RerollFailed::new(RollCharacteristic::Hit)),
                Ability::from(RerollOnes::new(RollCharacteristic::Hit)),
                Ability::from(Reroll::new(RollCharacteristic::Hit)),
            ],
        };
        assert_eq!(
            abilities.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(Reroll::new(RollCharacteristic::Hit)))
        );
    }

    #[test]
    fn reroll_ability_reroll_failed_found() {
        let abilities = AbilityManager {
            items: vec![
                Ability::from(RerollFailed::new(RollCharacteristic::Hit)),
                Ability::from(RerollOnes::new(RollCharacteristic::Hit)),
            ],
        };
        assert_eq!(
            abilities.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(RerollFailed::new(RollCharacteristic::Hit)))
        );
    }

    #[test]
    fn reroll_ability_reroll_ones_found() {
        let abilities = AbilityManager {
            items: vec![Ability::from(RerollOnes::new(RollCharacteristic::Hit))],
        };
        assert_eq!(
            abilities.reroll_ability(RollCharacteristic::Hit),
            Some(&Ability::from(RerollOnes::new(RollCharacteristic::Hit)))
        );
    }
}
