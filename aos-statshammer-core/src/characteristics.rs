use crate::dice::D6;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Value {
    pub value: u8,
    pub bonus: Option<i16>,
}
impl Value {
    pub(crate) fn unmodified(&self) -> u8 {
        self.value
    }

    pub(crate) fn modified(&self) -> u8 {
        (self.unmodified() as i16 + self.bonus.unwrap_or(0)).max(0) as u8
    }
}
impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self { value, bonus: None }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RerollType {
    Ones,
    Failed,
    Any,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RollTarget {
    pub value: u8,
    pub bonus: Option<i16>,
    pub reroll: Option<RerollType>,
}
impl RollTarget {
    pub(crate) fn unmodified(&self) -> u8 {
        self.value
    }

    pub(crate) fn modified(&self) -> u8 {
        (self.unmodified() as i16 - self.bonus.unwrap_or(0)).max(0) as u8
    }

    pub(crate) fn average(&self, count: f64) -> f64 {
        let rolls = count + (count * self.average_rerolls());
        rolls * D6.probability(self.modified())
    }

    fn average_rerolls(&self) -> f64 {
        match self.reroll {
            None => 0.0,
            Some(RerollType::Ones) => 1.0 / 6.0,
            Some(RerollType::Failed) => {
                D6.inverse_probability(self.modified().min(self.unmodified()))
            }
            Some(RerollType::Any) => D6.inverse_probability(self.modified()),
        }
    }
}
impl From<u8> for RollTarget {
    fn from(value: u8) -> Self {
        Self {
            value,
            bonus: None,
            reroll: None,
        }
    }
}
impl Add<i16> for RollTarget {
    type Output = RollTarget;
    fn add(self, rhs: i16) -> Self::Output {
        let mut result = self;
        result.bonus = Some(self.bonus.unwrap_or(0) + rhs);
        result
    }
}
impl Sub<i16> for RollTarget {
    type Output = RollTarget;
    fn sub(self, rhs: i16) -> Self::Output {
        let mut result = self;
        result.bonus = Some(self.bonus.unwrap_or(0) - rhs);
        result
    }
}
impl Default for RollTarget {
    fn default() -> Self {
        Self {
            value: 7,
            bonus: None,
            reroll: None,
        }
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    fn test_no_bonus() {
        let value = Value {
            value: 3,
            bonus: None,
        };
        assert_eq!(value.unmodified(), 3);
        assert_eq!(value.modified(), 3);
    }

    #[test]
    fn test_with_bonus() {
        let value = Value {
            value: 3,
            bonus: Some(1),
        };
        assert_eq!(value.unmodified(), 3);
        assert_eq!(value.modified(), 4);
    }
}

#[cfg(test)]
mod roll_target_tests {
    use super::*;
    use approx::assert_relative_eq;

    const MAX_RELATIVE: f64 = 0.001;

    #[test]
    fn test_no_bonus() {
        let target = RollTarget {
            value: 3,
            bonus: None,
            reroll: None,
        };
        assert_eq!(target.unmodified(), 3);
        assert_eq!(target.modified(), 3);
    }

    #[test]
    fn test_with_bonus() {
        let target = RollTarget {
            value: 3,
            bonus: Some(1),
            reroll: None,
        };
        assert_eq!(target.unmodified(), 3);
        assert_eq!(target.modified(), 2);
    }

    #[test]
    fn test_average_basic() {
        let target = RollTarget {
            value: 4,
            bonus: None,
            reroll: None,
        };
        assert_relative_eq!(target.average(3.0), 1.5);
    }

    #[test]
    fn test_average_with_bonus() {
        let target = RollTarget {
            value: 4,
            bonus: Some(1),
            reroll: None,
        };
        assert_relative_eq!(target.average(3.0), 2.0);
    }

    #[test]
    fn test_average_with_reroll_ones() {
        let target = RollTarget {
            value: 4,
            bonus: Some(1),
            reroll: Some(RerollType::Ones),
        };
        assert_relative_eq!(target.average(3.0), 2.333, max_relative = MAX_RELATIVE);
    }

    #[test]
    fn test_average_with_reroll_failed() {
        let target = RollTarget {
            value: 4,
            bonus: Some(1),
            reroll: Some(RerollType::Failed),
        };
        assert_relative_eq!(target.average(3.0), 2.667, max_relative = MAX_RELATIVE);
    }

    #[test]
    fn test_average_with_reroll_any() {
        let target = RollTarget {
            value: 4,
            bonus: Some(1),
            reroll: Some(RerollType::Any),
        };
        assert_relative_eq!(target.average(3.0), 2.667, max_relative = MAX_RELATIVE);
    }
}
