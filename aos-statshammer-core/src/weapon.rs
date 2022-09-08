use crate::{
    abilities::{weapon::*, RollCharacteristic},
    DiceNotation,
};
use derive_builder::Builder;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `Weapon` struct represents a single weapon profile that belongs to an Age of Sigmar unit and
/// includes all of the profile characteristics for it.
#[derive(Debug, PartialEq, Eq, Builder)]
#[builder(derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weapon {
    /// The number of models that are using this weapon profile
    #[builder(default = "1")]
    pub models: u32,

    /// The number of attacks that are made be **each** model wielding this weapon
    #[builder(try_setter, setter(into))]
    pub attacks: DiceNotation,

    /// Attacks using this weapon must roll this value or higher to hit
    pub to_hit: u32,

    /// Attacks using this weapon must roll this value or higher to wound
    pub to_wound: u32,

    /// Attacks using this weapon will reduce the save of the opposing unit by this value
    #[builder(default = "0")]
    pub rend: u32,

    /// The amount of damage that will be inflicted by a **single** unsaved wound
    #[builder(try_setter, setter(into))]
    pub damage: DiceNotation,

    /// A list of abilities that this weapon contains
    #[builder(default = "vec![]", setter(custom))]
    #[cfg_attr(feature = "serde", serde(default))]
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

    /// Return a new [`WeaponBuilder`] used for creating a new [`Weapon`] instance.
    ///
    /// A note that there may be a slight performance penalty for using the builder, depending on
    /// the number of abilities that you have.
    ///
    /// # Examples
    ///
    /// ## Valid Fully Qualified Weapon
    ///
    /// A general example with all fields defined, as well as, having some abilities added
    ///
    /// ```
    /// # use aos_statshammer_core::Weapon;
    /// # use aos_statshammer_core::abilities::{
    /// #   RerollType, RollCharacteristic, weapon::{Bonus, Reroll}
    /// # };
    /// #
    /// let weapon = Weapon::builder()
    ///     .models(5)
    ///     .attacks(2)
    ///     .to_hit(3)
    ///     .to_wound(4)
    ///     .rend(1)
    ///     .damage(2)
    ///     .ability(Reroll {
    ///         characteristic: RollCharacteristic::Hit,
    ///         reroll_type: RerollType::Any,
    ///     })
    ///     .ability(Bonus {
    ///         characteristic: RollCharacteristic::Hit.into(),
    ///         value: 2.into(),
    ///      })
    ///     .build();
    ///
    /// assert!(matches!(weapon, Ok(Weapon { .. })));
    /// ```
    ///
    /// ## Valid Using Defaults and TryFrom
    ///
    /// There are also defaults defined for `models`, `rend`, and `abilities`.
    /// We can also use `try_attacks` and `try_damage` to use the `TryFrom` implementation for
    /// [`DiceNotation`]
    ///
    /// ```
    /// # use aos_statshammer_core::Weapon;
    /// # use aos_statshammer_core::abilities::{
    /// #   RerollType, RollCharacteristic, weapon::{Bonus, Reroll}
    /// # };
    /// #
    /// let weapon = Weapon::builder()
    ///     .try_attacks("d6").unwrap()
    ///     .to_hit(3)
    ///     .to_wound(4)
    ///     .try_damage("2d3 + 2").unwrap()
    ///     .build();
    ///
    /// assert!(matches!(weapon, Ok(Weapon { .. })));
    /// ```
    ///
    /// ## Invalid Weapon
    ///
    /// If we do not define all of the fields which are required, you will get an error back.
    ///
    /// ```
    /// # use aos_statshammer_core::Weapon;
    /// # use aos_statshammer_core::abilities::{
    /// #   RerollType, RollCharacteristic, weapon::{Bonus, Reroll}
    /// # };
    /// #
    /// let weapon = Weapon::builder().models(5).build(); // Missing fields
    /// assert!(weapon.is_err());
    /// ```
    pub fn builder() -> WeaponBuilder {
        WeaponBuilder::default()
    }

    pub fn reroll_ability(&self, phase: RollCharacteristic) -> Option<&Reroll> {
        self.abilities
            .iter()
            .filter_map(|ability| match ability {
                Ability::Reroll(a) if a.characteristic == phase => Some(a),
                _ => None,
            })
            .max_by_key(|a| a.reroll_type)
    }
}

impl WeaponBuilder {
    pub fn ability<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Ability>,
    {
        let av = &mut self.abilities;
        match av {
            Some(v) => v.push(value.into()),
            _ => self.abilities = Some(vec![value.into()]),
        }
        self
    }
}

// ========================================
//                UNIT TESTS
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abilities::RerollType;

    #[test]
    fn weapon_builder_shortcut() {
        let builder = Weapon::builder();
        assert_eq!(builder, WeaponBuilder::default());
    }

    #[test]
    fn weapon_builder_basic() {
        let output = WeaponBuilder::default()
            .models(5)
            .attacks(DiceNotation::from(2))
            .to_hit(3)
            .to_wound(4)
            .rend(1)
            .damage(DiceNotation::from(2))
            .build()
            .unwrap();
        assert_eq!(
            output,
            Weapon {
                models: 5,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(2),
                abilities: vec![]
            }
        )
    }

    #[test]
    fn weapon_builder_with_into_and_abilities() {
        let output = WeaponBuilder::default()
            .models(5)
            .attacks(2)
            .to_hit(3)
            .to_wound(4)
            .rend(1)
            .damage(2)
            .ability(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Any,
            })
            .ability(Bonus {
                characteristic: RollCharacteristic::Hit.into(),
                value: 2.into(),
            })
            .build()
            .unwrap();
        assert_eq!(
            output,
            Weapon {
                models: 5,
                attacks: DiceNotation::from(2),
                to_hit: 3,
                to_wound: 4,
                rend: 1,
                damage: DiceNotation::from(2),
                abilities: vec![
                    Ability::from(Reroll {
                        characteristic: RollCharacteristic::Hit,
                        reroll_type: RerollType::Any,
                    }),
                    Ability::from(Bonus {
                        characteristic: RollCharacteristic::Hit.into(),
                        value: 2.into(),
                    })
                ]
            }
        )
    }

    #[test]
    fn weapon_builder_using_defaults_and_try_into() {
        let output = WeaponBuilder::default()
            .try_attacks("d6")
            .unwrap()
            .to_hit(3)
            .to_wound(4)
            .try_damage("2d6 + 1")
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            output,
            Weapon {
                models: 1,
                attacks: DiceNotation::try_from("d6").unwrap(),
                to_hit: 3,
                to_wound: 4,
                rend: 0,
                damage: DiceNotation::try_from("2d6 + 1").unwrap(),
                abilities: vec![]
            }
        )
    }

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
        let weapon = create_weapon!(vec![Ability::from(Reroll {
            characteristic: RollCharacteristic::Wound,
            reroll_type: RerollType::Any
        })]);
        assert_eq!(weapon.reroll_ability(RollCharacteristic::Hit), None);
    }

    #[test]
    fn reroll_ability_reroll_found() {
        let weapon = create_weapon!(vec![
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Any
            }),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Any
            })
        );
    }

    #[test]
    fn reroll_ability_reroll_failed_found() {
        let weapon = create_weapon!(vec![
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed
            }),
            Ability::from(Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones
            }),
        ]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Failed,
            })
        );
    }

    #[test]
    fn reroll_ability_reroll_ones_found() {
        let weapon = create_weapon!(vec![Ability::from(Reroll {
            characteristic: RollCharacteristic::Hit,
            reroll_type: RerollType::Ones
        })]);
        assert_eq!(
            weapon.reroll_ability(RollCharacteristic::Hit),
            Some(&Reroll {
                characteristic: RollCharacteristic::Hit,
                reroll_type: RerollType::Ones,
            })
        );
    }
}
