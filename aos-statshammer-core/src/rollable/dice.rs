use super::{Rollable, Roller};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// TODO: Move to a lower sized int (e.g: u8)

/// A `Dice` struct repesents a dice with a set number of sides. It can have any quantity of
/// these dice sharing the same number of sides (e.g: equivalent of `d6` or `2d6`).
///
/// If you need a combination of different sided dice (or constants) then you will need to use a
/// [`DiceNotation`](crate::DiceNotation) struct.
///
/// # Examples
///
/// ## Basic example using the `new` constructor
///
/// ```
/// use aos_statshammer_core::Dice;
/// // Equivalent of 2d6
/// let dice = Dice::new(6, 2);
/// ```
///
/// ## Example valid dice string
///
/// ```
/// use aos_statshammer_core::Dice;
///
/// let dn = Dice::try_from("2d6");
/// assert!(dn.is_ok());
/// assert_eq!(dn, Ok(Dice {sides: 6, quantity: 2}));
/// ```
///
/// ## Example invalid dice string
///
/// ```
/// use aos_statshammer_core::Dice;
///
/// let dn = Dice::try_from("invalid");
/// assert!(dn.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dice {
    pub sides: u32,
    pub quantity: u32,
}

impl Dice {
    /// Create a `Dice` from a given number of sides and quantity.
    ///
    /// # Arguments
    ///
    /// * `sides` - The number of sides for the dice
    /// * `quantity` - The quantity of dice with the given `sides`
    ///
    /// # Examples
    ///
    /// ```
    /// use aos_statshammer_core::Dice;
    /// // Equivalent of 2d6
    /// let dice = Dice::new(6, 2);
    /// ```
    pub fn new(sides: u32, quantity: u32) -> Self {
        Self { sides, quantity }
    }

    /// Shorthand for creating a single `Dice` with a given number of sides
    ///
    /// # Examples
    ///
    /// ```
    /// # use aos_statshammer_core::Dice;
    /// let dice = Dice::d(3);
    /// assert_eq!(dice, Dice {sides: 3, quantity: 1});
    /// ```
    pub fn d(sides: u32) -> Self {
        Self { sides, quantity: 1 }
    }

    /// Create a `Dice` with 6 sides (and quantity 1)
    pub fn d6() -> Self {
        Self {
            sides: 6,
            quantity: 1,
        }
    }
}

impl Rollable for Dice {
    /// Get the minimum value that can be obtained (with a minimum of `0`)
    fn min(&self) -> u32 {
        self.quantity
    }

    fn max(&self) -> u32 {
        self.quantity * self.sides
    }

    fn average(&self) -> f32 {
        let single_average = (self.sides as f32 + 1.0) / 2.0;
        (self.quantity as f32) * single_average
    }

    fn roll<T: Roller>(&self, roller: &T) -> u32 {
        (0..self.quantity).fold(0, |acc, _| acc + roller.roll(self.sides))
    }
}

impl TryFrom<&str> for Dice {
    type Error = &'static str;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref DICE_RE: Regex = Regex::new(r"^(\d+)?[dD](\d+)$").unwrap();
        }
        match DICE_RE.captures(data.trim()) {
            Some(caps) => {
                let get_int = |i: usize, default: u32| {
                    caps.get(i)
                        .map_or(default, |m| m.as_str().parse::<u32>().unwrap())
                };
                let quantity: u32 = get_int(1, 1);
                let sides: u32 = get_int(2, 6);
                Ok(Self { sides, quantity })
            }
            _ => Err("Invalid dice string provided"),
        }
    }
}

impl fmt::Display for Dice {
    /// Formats the `Dice` value using the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use aos_statshammer_core::Dice;
    ///
    /// let dice = Dice::new(6, 2);
    /// assert_eq!(format!("{}", dice), "2d6");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.quantity > 1 {
            write!(f, "{}d{}", self.quantity, self.sides)
        } else {
            write!(f, "d{}", self.sides)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rollable::MockRoller;
    use mockall::predicate;
    use test_case::test_case;

    #[test_case(Dice {sides: 3, quantity: 1} => 1 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => 1 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => 2 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => 4 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => 6 ; "6d10")]
    fn min(dice: Dice) -> u32 {
        dice.min()
    }

    #[test_case(Dice {sides: 3, quantity: 1} => 3 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => 6 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => 12 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => 32 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => 60 ; "6d10")]
    fn max(dice: Dice) -> u32 {
        dice.max()
    }

    #[test_case(Dice {sides: 3, quantity: 1} => 2.0 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => 3.5 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => 7.0 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => 18.0 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => 33.0 ; "6d10")]
    fn average(dice: Dice) -> f32 {
        dice.average()
    }

    #[test]
    fn roll_single() {
        let mut mock = MockRoller::default();
        mock.expect_roll()
            .with(predicate::eq(6))
            .times(1)
            .returning(|_| 4);
        let dice = Dice {
            sides: 6,
            quantity: 1,
        };
        assert_eq!(dice.roll(&mock), 4);
    }

    #[test]
    fn roll_multiple() {
        let mut mock = MockRoller::default();
        mock.expect_roll()
            .with(predicate::eq(6))
            .times(3)
            .returning(|_| 4);
        let dice = Dice {
            sides: 6,
            quantity: 3,
        };
        assert_eq!(dice.roll(&mock), 12);
    }

    #[test_case("d6" => Ok(Dice {sides: 6, quantity: 1}) ; "lowercase d6")]
    #[test_case("D6" => Ok(Dice {sides: 6, quantity: 1}) ; "uppercase D6")]
    #[test_case("2d6" => Ok(Dice {sides: 6, quantity: 2}) ; "uppercase 2d6")]
    #[test_case(" 4d8 " => Ok(Dice {sides: 8, quantity: 4}) ; "hase spaces")]
    #[test_case("2d6" => Ok(Dice {sides: 6, quantity: 2}) ; "lowercase 4d8")]
    #[test_case("invalid" => matches Err(_) ; "invalid")]
    fn from(data: &str) -> Result<Dice, &str> {
        Dice::try_from(data)
    }
}
