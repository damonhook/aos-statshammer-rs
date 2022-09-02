use crate::abilities::opponent::*;
use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Builder)]
#[builder(derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Opponent {
    /// A list of abilities that this opponent has
    #[builder(default = "vec![]", setter(custom))]
    pub abilities: Vec<OpponentAbility>,
}
impl Opponent {
    pub fn new(abilities: Vec<OpponentAbility>) -> Self {
        Self { abilities }
    }

    pub fn builder() -> OpponentBuilder {
        OpponentBuilder::default()
    }

    pub fn reroll_ability(&self) -> Option<&SaveReroll> {
        self.abilities
            .iter()
            .filter_map(|ability| match ability {
                OpponentAbility::SaveReroll(a) => Some(a),
                _ => None,
            })
            .max_by_key(|a| a.reroll_type)
    }

    /// Returns the best [`Ward`] ability that this opponent has
    pub fn ward(&self) -> Option<&Ward> {
        self.abilities
            .iter()
            .filter_map(|ability| match ability {
                OpponentAbility::Ward(a) => Some(a),
                _ => None,
            })
            .max_by(|&x, &y| x.on.cmp(&y.on))
    }

    /// Returns whether this opponent has any [`Ethereal`] abilities
    pub fn is_ethereal(&self) -> bool {
        self.abilities
            .iter()
            .any(|a| matches!(a, OpponentAbility::Ethereal { .. }))
    }
}

impl OpponentBuilder {
    pub fn ability<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<OpponentAbility>,
    {
        let av = &mut self.abilities;
        match av {
            Some(v) => v.push(value.into()),
            _ => self.abilities = Some(vec![value.into()]),
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abilities::RerollType;

    #[test]
    fn opponent_builder() {
        let output = Opponent::builder()
            .ability(SaveReroll {
                reroll_type: RerollType::Any,
            })
            .ability(Ethereal {})
            .build()
            .unwrap();
        assert_eq!(
            output,
            Opponent {
                abilities: vec![
                    OpponentAbility::from(SaveReroll {
                        reroll_type: RerollType::Any,
                    }),
                    OpponentAbility::from(Ethereal {})
                ]
            }
        )
    }
}
