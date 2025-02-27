use crate::rerolls::RerollType;
use crate::serde_utils::*;
use aos_statshammer_core::weapon;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct WeaponConfig {
    characteristics: Characteristics,
    #[serde(default)]
    abilities: Abilities,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Characteristics {
    attack: u8,
    hit: u8,
    wound: u8,
    rend: u8,
    damage: u8,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Abilities {
    #[serde(default)]
    bonus: Vec<BonusAbility>,
    #[serde(default)]
    reroll: Vec<RerollAbility>,
    #[serde(default)]
    exploding: Vec<ExplodingAbility>,
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ValueCharacteristic {
    Attacks,
    Hit,
    Wound,
    Rend,
    Damage,
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RollCharacteristic {
    Hit,
    Wound,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub(crate) struct BonusAbility {
    to: ValueCharacteristic,
    #[serde(default = "default_i16::<1>")]
    amount: i16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RerollAbility {
    to: RollCharacteristic,
    #[serde(rename = "type")]
    reroll_type: RerollType,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub(crate) struct ExplodingAbility {
    to: RollCharacteristic,
    #[serde(default = "default_u8::<6>")]
    on: u8,
    #[serde(default = "default_bool::<false>")]
    unmodified: bool,
    #[serde(default = "default_u8::<1>")]
    extra: u8,
}
impl From<ExplodingAbility> for weapon::ExplodingAbility {
    fn from(value: ExplodingAbility) -> Self {
        Self {
            on: value.on,
            unmodified: value.unmodified,
            extra: value.extra,
        }
    }
}

impl TryInto<weapon::Weapon> for WeaponConfig {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<weapon::Weapon, Self::Error> {
        let mut builder = weapon::WeaponBuilder::default();
        builder
            .attacks(weapon::Attacks {
                value: self.characteristics.attack,
                bonus: self.bonus_to(ValueCharacteristic::Attacks),
            })
            .hit(weapon::Hit {
                value: self.characteristics.hit,
                bonus: self.bonus_to(ValueCharacteristic::Hit),
                reroll: self.reroll_to(RollCharacteristic::Hit).map(Into::into),
                exploding: self.exploding_to(RollCharacteristic::Hit).map(Into::into),
            })
            .wound(weapon::Wound {
                value: self.characteristics.wound,
                bonus: self.bonus_to(ValueCharacteristic::Wound),
                reroll: self.reroll_to(RollCharacteristic::Wound).map(Into::into),
                exploding: self.exploding_to(RollCharacteristic::Wound).map(Into::into),
            })
            .rend(weapon::Rend {
                value: self.characteristics.rend,
                bonus: self.bonus_to(ValueCharacteristic::Rend),
            })
            .damage(weapon::Damage {
                value: self.characteristics.damage,
                bonus: self.bonus_to(ValueCharacteristic::Damage),
            });
        builder.build().map_err(Into::into)
    }
}

impl WeaponConfig {
    fn bonus_to(&self, to: ValueCharacteristic) -> i16 {
        self.abilities
            .bonus
            .iter()
            .filter(|a| a.to == to)
            .fold(0, |acc, a| acc + a.amount)
    }

    fn reroll_to(&self, to: RollCharacteristic) -> Option<RerollType> {
        self.abilities
            .reroll
            .iter()
            .filter(|a| a.to == to)
            .map(|a| a.reroll_type)
            .max()
    }

    fn exploding_to(&self, to: RollCharacteristic) -> Option<ExplodingAbility> {
        self.abilities
            .exploding
            .iter()
            .find(|a| a.to == to)
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_characteristics() -> Characteristics {
        Characteristics {
            attack: 2,
            hit: 3,
            wound: 4,
            rend: 0,
            damage: 1,
        }
    }

    #[test]
    fn bonus_to_sums_many_together() {
        let config = WeaponConfig {
            characteristics: basic_characteristics(),
            abilities: Abilities {
                bonus: vec![
                    BonusAbility {
                        to: ValueCharacteristic::Attacks,
                        amount: 1,
                    },
                    BonusAbility {
                        to: ValueCharacteristic::Attacks,
                        amount: 2,
                    },
                ],
                ..Default::default()
            },
        };
        assert_eq!(config.bonus_to(ValueCharacteristic::Attacks), 3);
    }

    #[test]
    fn reroll_picks_best_option() {
        let config = WeaponConfig {
            characteristics: basic_characteristics(),
            abilities: Abilities {
                reroll: vec![
                    RerollAbility {
                        to: RollCharacteristic::Hit,
                        reroll_type: RerollType::Failed,
                    },
                    RerollAbility {
                        to: RollCharacteristic::Hit,
                        reroll_type: RerollType::Ones,
                    },
                    RerollAbility {
                        to: RollCharacteristic::Wound,
                        reroll_type: RerollType::Ones,
                    },
                    RerollAbility {
                        to: RollCharacteristic::Wound,
                        reroll_type: RerollType::Any,
                    },
                    RerollAbility {
                        to: RollCharacteristic::Wound,
                        reroll_type: RerollType::Failed,
                    },
                ],
                ..Default::default()
            },
        };
        assert_eq!(
            config.reroll_to(RollCharacteristic::Hit),
            Some(RerollType::Failed)
        );
        assert_eq!(
            config.reroll_to(RollCharacteristic::Wound),
            Some(RerollType::Any)
        );
    }

    #[test]
    fn create_weapon_basic() {
        let config = WeaponConfig {
            characteristics: Characteristics {
                attack: 2,
                hit: 3,
                wound: 4,
                rend: 1,
                damage: 1,
            },
            abilities: Default::default(),
        };
        let output: weapon::Weapon = config.try_into().unwrap();
        assert_eq!(
            output,
            weapon::Weapon {
                attacks: weapon::Attacks {
                    value: 2,
                    ..Default::default()
                },
                hit: weapon::Hit {
                    value: 3,
                    ..Default::default()
                },
                wound: weapon::Wound {
                    value: 4,
                    ..Default::default()
                },
                rend: weapon::Rend {
                    value: 1,
                    ..Default::default()
                },
                damage: weapon::Damage {
                    value: 1,
                    ..Default::default()
                },
            }
        )
    }
}
