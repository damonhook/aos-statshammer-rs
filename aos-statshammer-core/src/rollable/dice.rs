use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::fmt;

use super::Rollable;

/// A `Dice` struct repesents a dice with a set number of sides. It can have any quantity of 
/// these dice sharing the same number of sides (e.g: equivalent of `d6` or `2d6`).
/// 
/// If you need a combination of different sided dice (or constants) then you will need to use a
/// `DiceNotation` struct.
#[derive(Debug, Clone, PartialEq)]
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
    /// use aos_statshammer_core::rollable::Dice;
    /// // Equivalent of 2d6
    /// let dice = Dice::new(6, 2);
    /// ```
    pub fn new(sides: u32, quantity: u32) -> Self {
        Self { sides, quantity }
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

    fn roll(&self) -> u32 {
        rand::thread_rng().gen_range(1..=self.sides)
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.quantity > 1 {
            write!(f, "{}d{}", self.quantity, self.sides)
        } else {
            write!(f, "d{}", self.sides)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Dice {sides: 3, quantity: 1} => it eq 1 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => it eq 1 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => it eq 2 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => it eq 4 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => it eq 6 ; "6d10")]
    fn min(dice: Dice) -> u32 {
        dice.min()
    }

    #[test_case(Dice {sides: 3, quantity: 1} => it eq 3 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => it eq 6 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => it eq 12 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => it eq 32 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => it eq 60 ; "6d10")]
    fn max(dice: Dice) -> u32 {
        dice.max()
    }

    #[test_case(Dice {sides: 3, quantity: 1} => it eq 2.0 ; "d3")]
    #[test_case(Dice {sides: 6, quantity: 1} => it eq 3.5 ; "d6")]
    #[test_case(Dice {sides: 6, quantity: 2} => it eq 7.0 ; "2d6")]
    #[test_case(Dice {sides: 8, quantity: 4} => it eq 18.0 ; "4d8")]
    #[test_case(Dice {sides: 10, quantity: 6} => it eq 33.0 ; "6d10")]
    fn average(dice: Dice) -> f32 {
        dice.average()
    }

    #[test_case("d6" => it eq Ok(Dice {sides: 6, quantity: 1}) ; "lowercase d6")]
    #[test_case("D6" => it eq Ok(Dice {sides: 6, quantity: 1}) ; "uppercase D6")]
    #[test_case("2d6" => it eq Ok(Dice {sides: 6, quantity: 2}) ; "uppercase 2d6")]
    #[test_case(" 4d8 " => it eq Ok(Dice {sides: 8, quantity: 4}) ; "hase spaces")]
    #[test_case("2d6" => it eq Ok(Dice {sides: 6, quantity: 2}) ; "lowercase 4d8")]
    #[test_case("invalid" => matches Err(_) ; "invalid")]
    fn from(data: &str) -> Result<Dice, &str> {
        Dice::try_from(data)
    }
}
