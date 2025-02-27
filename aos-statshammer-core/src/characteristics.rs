use crate::dice::D6;

// While these `macro_rules!` helpers could be turned into a proc-macro,
// going to leave them as is for the time being.

pub trait Characteristic {
    fn bonus(&self) -> i16;
    fn unmodified(&self) -> u8;
    fn modified(&self) -> u8;
}

macro_rules! impl_characteristic {
    ($bound: ty, $value: ident, $bonus: ident) => {
        impl_characteristic!($bound, $value, $bonus, +);
    };
    ($bound: ty, $value: ident, $bonus: ident, $op: tt) => {
        impl Characteristic for $bound {
            fn bonus(&self) -> i16 {
                self.$bonus
            }

            fn unmodified(&self) -> u8 {
                self.$value
            }

            fn modified(&self) -> u8 {
                 (self.unmodified() as i16 $op self.bonus()).max(0) as u8
            }
        }

        impl From<u8> for $bound {
            fn from(value: u8) -> Self {
                Self {
                    $value: value,
                    ..Default::default()
                }
            }
        }

        impl std::ops::Add<i16> for $bound {
            type Output = $bound;
            fn add(self, rhs: i16) -> Self::Output {
                let mut result = self;
                result.$bonus += rhs;
                result
            }
        }
        impl std::ops::Sub<i16> for $bound {
            type Output = $bound;
            fn sub(self, rhs: i16) -> Self::Output {
                let mut result = self;
                result.$bonus -= rhs;
                result
            }
        }
    };
}
pub(crate) use impl_characteristic;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RerollType {
    Ones,
    Failed,
    Any,
}
pub trait Reroll: Characteristic {
    fn reroll_type(&self) -> Option<RerollType>;
    fn reroll_probability(&self) -> f64 {
        match self.reroll_type() {
            None => 0.0,
            Some(RerollType::Ones) => 1.0 / 6.0,
            Some(RerollType::Failed) => {
                D6.inverse_probability(self.modified().min(self.unmodified()))
            }
            Some(RerollType::Any) => D6.inverse_probability(self.modified()),
        }
    }
}

macro_rules! impl_reroll {
    ($bound: ty, $prop: ident) => {
        impl Reroll for $bound {
            fn reroll_type(&self) -> Option<RerollType> {
                self.$prop
            }
        }
    };
}
pub(crate) use impl_reroll;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ExplodingAbility {
    pub on: u8,
    pub unmodified: bool,
    pub extra: u8,
}
impl Default for ExplodingAbility {
    fn default() -> Self {
        Self {
            on: 6,
            unmodified: true,
            extra: 1,
        }
    }
}
pub trait Exploding: Characteristic {
    fn extra_probability(&self) -> f64;
}
macro_rules! impl_exploding {
    ($bound: ty, $prop: ident) => {
        impl Exploding for $bound {
            fn extra_probability(&self) -> f64 {
                if let Some(ab) = self.$prop {
                    let roll_target = match ab.unmodified {
                        true => ab.on,
                        false => (ab.on as i16 - self.bonus()).max(0) as u8,
                    };
                    ab.extra as f64 * D6.probability(roll_target)
                } else {
                    0.0
                }
            }
        }
    };
}
pub(crate) use impl_exploding;

#[cfg(test)]
mod test_characteristic {
    use super::*;

    #[derive(Debug, Default)]
    struct ExamplePositive {
        value: u8,
        bonus: i16,
    }
    impl_characteristic!(ExamplePositive, value, bonus);

    #[derive(Debug, Default)]
    struct ExampleNegative {
        value: u8,
        bonus: i16,
    }
    impl_characteristic!(ExampleNegative, value, bonus, -);

    #[test]
    fn positive() {
        let item = ExamplePositive { value: 3, bonus: 1 };
        assert_eq!(item.bonus(), 1);
        assert_eq!(item.unmodified(), 3);
        assert_eq!(item.modified(), 4);
    }

    #[test]
    fn negative() {
        let item = ExampleNegative { value: 3, bonus: 1 };
        assert_eq!(item.bonus(), 1);
        assert_eq!(item.unmodified(), 3);
        assert_eq!(item.modified(), 2);
    }
}

#[cfg(test)]
mod test_reroll {
    use super::*;
    use approx::assert_relative_eq;

    #[derive(Debug, Default)]
    struct Example {
        value: u8,
        bonus: i16,
        reroll: Option<RerollType>,
    }
    impl_characteristic!(Example, value, bonus, -);
    impl_reroll!(Example, reroll);

    #[test]
    fn no_reroll() {
        let item = Example {
            value: 4,
            bonus: 1,
            reroll: None,
        };
        assert_eq!(item.reroll_type(), None);
        assert_eq!(item.reroll_probability(), 0.0);
    }

    #[test]
    fn reroll_ones() {
        let item = Example {
            value: 4,
            bonus: 1,
            reroll: Some(RerollType::Ones),
        };
        assert_eq!(item.reroll_type(), Some(RerollType::Ones));
        assert_relative_eq!(item.reroll_probability(), 0.1667, max_relative = 0.001);
    }

    #[test]
    fn reroll_failed() {
        let item = Example {
            value: 4,
            bonus: 1,
            reroll: Some(RerollType::Failed),
        };
        assert_eq!(item.reroll_type(), Some(RerollType::Failed));
        assert_relative_eq!(item.reroll_probability(), 0.3333, max_relative = 0.001);
    }

    #[test]
    fn reroll_any() {
        let item = Example {
            value: 4,
            bonus: 1,
            reroll: Some(RerollType::Any),
        };
        assert_eq!(item.reroll_type(), Some(RerollType::Any));
        assert_relative_eq!(item.reroll_probability(), 0.3333, max_relative = 0.001);
    }

    #[test]
    fn reroll_failed_with_negative_bonus() {
        let mut item = Example {
            value: 4,
            bonus: -1,
            reroll: None,
        };
        assert_relative_eq!(item.reroll_probability(), 0.0, max_relative = 0.001);
        item.reroll = Some(RerollType::Any);
        assert_relative_eq!(item.reroll_probability(), 0.6667, max_relative = 0.001);
        item.reroll = Some(RerollType::Failed);
        assert_relative_eq!(item.reroll_probability(), 0.5, max_relative = 0.001);
    }
}
