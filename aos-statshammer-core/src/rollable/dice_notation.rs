use super::Dice;
use super::Rollable;
use std::fmt::Write as _; // import without risk of name clashing
use std::{
    cmp, fmt,
    ops::{AddAssign, SubAssign},
};

/// A `DiceNotation` struct represents an expression containing various dice and constant values 
/// (e.g: `2d6 + d3 - 2`) while providing some convenience functions for them.
#[derive(Debug, Clone, PartialEq)]
pub struct DiceNotation {
    pub additions: Vec<Dice>,
    pub subtractions: Vec<Dice>,
    pub constant: i32,
}

impl DiceNotation {
    /// Return a `DiceNotation` given the `Dice` and constant components.
    /// 
    /// # Arguments
    /// 
    /// * `additions` - A vector containing all of the positive `Dice` components
    /// * `subtractions` - A vector containing all of the negative `Dice` components
    /// * `constant` - A constant value (positive or negative) for the notation (e.g: `2` or `-1`)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use aos_statshammer_core::rollable::{Dice, DiceNotation};
    /// // Equivalent of 2d6 - d3 + 2
    /// let dn = DiceNotation::new(
    ///     vec![Dice {sides: 6, quantity: 2}],
    ///     vec![Dice {sides: 3, quantity: 1}],
    ///     2
    /// );
    /// ```
    pub fn new(additions: Vec<Dice>, subtractions: Vec<Dice>, constant: i32) -> Self {
        Self {
            additions,
            subtractions,
            constant,
        }
    }

    /// Return an empty `DiceNotation`
    pub fn empty() -> Self {
        Self {
            additions: vec![],
            subtractions: vec![],
            constant: 0,
        }
    }
}

impl Rollable for DiceNotation {
    /// Get the minimum value that can be obtained (with a minimum of `0`)
    fn min(&self) -> u32 {
        let mut min: i32 = 0;
        for addition in &self.additions {
            min += addition.min() as i32;
        }
        for subtraction in &self.subtractions {
            min -= subtraction.max() as i32;
        }
        cmp::max(min + self.constant, 0) as u32
    }

    /// Get the maximum value that can be obtained (with a minimum of `0`)
    fn max(&self) -> u32 {
        let mut max: i32 = 0;
        for addition in &self.additions {
            max += addition.max() as i32;
        }
        for subtraction in &self.subtractions {
            max -= subtraction.min() as i32;
        }
        cmp::max(max + self.constant, 0) as u32
    }

    /// Get the average value for this expression (with a minimum of `0`)
    fn average(&self) -> f32 {
        let mut average: f32 = 0.0;
        for addition in &self.additions {
            average += addition.average();
        }
        for subtraction in &self.subtractions {
            average -= subtraction.average();
        }
        average += self.constant as f32;
        average.max(0.0)
    }

    /// Roll a "random" number given this notation (with a minimum of `0`)
    fn roll(&self) -> u32 {
        let mut value: i32 = 0;
        for addition in &self.additions {
            value += addition.roll() as i32;
        }
        for subtraction in &self.subtractions {
            value -= subtraction.roll() as i32;
        }
        value += self.constant;
        cmp::max(value, 0) as u32
    }
}

impl fmt::Display for DiceNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn dice_list_to_string(lst: &[Dice], sep: &str) -> String {
            let string_vec = lst.iter().map(Dice::to_string).collect::<Vec<String>>();
            string_vec.join(sep)
        }

        let mut display: String = dice_list_to_string(&self.additions, "+");
        if !self.subtractions.is_empty() {
            // display.push_str(&format!("-{}", dice_list_to_string(&self.subtractions, "-")))
            let _ = write!(display, "-{}", dice_list_to_string(&self.subtractions, "-"));
        }

        match self.constant.cmp(&0) {
            cmp::Ordering::Greater => {
                let prefix = if !display.is_empty() { "+" } else { "" };
                // display.push_str(&format!("{}{}", prefix, self.constant));
                let _ = write!(display, "{}{}", prefix, self.constant);
            },
            cmp::Ordering::Less => {
                display.push_str(&self.constant.to_string());
            }
            cmp::Ordering::Equal => {}
        }

        write!(f, "{}", display)
    }
}

impl AddAssign<DiceNotation> for DiceNotation {
    fn add_assign(&mut self, rhs: DiceNotation) {
        self.additions.extend(rhs.additions.to_vec());
        self.subtractions.extend(rhs.subtractions.to_vec());
        self.constant += rhs.constant;
    }
}

impl AddAssign<Dice> for DiceNotation {
    fn add_assign(&mut self, rhs: Dice) {
        self.additions.push(rhs);
    }
}
impl SubAssign<Dice> for DiceNotation {
    fn sub_assign(&mut self, rhs: Dice) {
        self.subtractions.push(rhs);
    }
}

impl AddAssign<i32> for DiceNotation {
    fn add_assign(&mut self, rhs: i32) {
        self.constant += rhs;
    }
}
impl SubAssign<i32> for DiceNotation {
    fn sub_assign(&mut self, rhs: i32) {
        self.constant -= rhs;
    }
}

impl From<i32> for DiceNotation {
    /// Create a `DiceNotation` from an integer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use aos_statshammer_core::rollable::DiceNotation;
    /// 
    /// let dn = DiceNotation::from(3);
    /// assert_eq!(dn, DiceNotation {additions: vec![], subtractions: vec![], constant: 3});
    /// ```
    fn from(constant: i32) -> Self {
        Self {
            additions: vec![],
            subtractions: vec![],
            constant,
        }
    }
}

impl From<Dice> for DiceNotation {
    /// Create a `DiceNotation` from a single `Dice`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use aos_statshammer_core::rollable::{DiceNotation, Dice};
    /// 
    /// let dn = DiceNotation::from(Dice {sides: 6, quantity: 2});
    /// assert_eq!(
    ///     dn, 
    ///     DiceNotation {
    ///         additions: vec![Dice {sides: 6, quantity: 2}],
    ///         subtractions: vec![],
    ///         constant: 0
    ///     }
    /// );
    /// ```
    fn from(dice: Dice) -> Self {
        Self {
            additions: vec![dice],
            subtractions: vec![],
            constant: 0,
        }
    }
}

impl TryFrom<&str> for DiceNotation {
    type Error = &'static str;

    /// Attempts to create a `DiceNotation` from a `&str`.
    /// 
    /// # Examples
    /// 
    /// ## Valid
    /// 
    /// ```
    /// use aos_statshammer_core::rollable::{DiceNotation, Dice};
    /// 
    /// let dn = DiceNotation::try_from("2d6 + d3 - 2");
    /// assert!(dn.is_ok());
    /// assert_eq!(dn, Ok(DiceNotation {
    ///      additions: vec![Dice {sides: 6, quantity: 2}, Dice {sides: 3, quantity: 1}],
    ///      subtractions: vec![],
    ///      constant: -2,
    /// }));
    /// ```
    /// 
    /// ## Invalid
    /// 
    /// ```
    /// use aos_statshammer_core::rollable::DiceNotation;
    /// 
    /// let dn = DiceNotation::try_from("invalid");
    /// assert!(dn.is_err());
    /// ```
    fn try_from(data: &str) -> Result<Self, Self::Error> {
        let mut additions: Vec<Dice> = vec![];
        let mut subtractions: Vec<Dice> = vec![];
        let mut constant: i32 = 0;

        let data: String = data.split_whitespace().collect::<String>();

        let items: Vec<&str> = data.split_inclusive(&['-', '+'][..]).collect();

        let mut is_addition: bool = true;
        for mut item in items {
            let next_is_addition = match item.chars().last() {
                Some(op @ '+') | Some(op @ '-') => {
                    item = item.strip_suffix(&['-', '+'][..]).unwrap_or(item);
                    op != '-'
                }
                _ => true,
            };

            if item.contains(&['d', 'D'][..]) {
                match Dice::try_from(item) {
                    Ok(dice) => {
                        if is_addition {
                            additions.push(dice);
                        } else {
                            subtractions.push(dice);
                        }
                    }
                    Err(_) => {
                        return Err("Invalid dice notation string provided")
                    },
                }
            } else {
                let val: i32 = item.parse().unwrap_or(0);
                if is_addition {
                    constant += val;
                } else {
                    constant -= val;
                }
            }

            is_addition = next_is_addition;
        }

        Ok(Self {
            additions,
            subtractions,
            constant,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn new() {
        let output = DiceNotation::new(
            vec![Dice::new(6, 2)], vec![Dice::new(6, 1)], 2
        );
        let expected = DiceNotation { 
            additions: vec![Dice::new(6, 2)], 
            subtractions: vec![Dice::new(6, 1)], 
            constant: 2
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn empty() {
        let output = DiceNotation::empty();
        let expected = DiceNotation { 
            additions: vec![], 
            subtractions: vec![], 
            constant: 0
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn add_assign_dice_notation() {
        let mut dn = DiceNotation::new(
            vec![Dice::new(6, 2)], 
            vec![Dice::new(6, 1)], 
            2
        );
        dn += DiceNotation::new(
            vec![Dice::new(8, 1), Dice::new(10, 2)], 
            vec![Dice::new(3, 1)], 
            -1
        );
        let expected = DiceNotation::new(
            vec![Dice::new(6, 2), Dice::new(8, 1), Dice::new(10, 2)], 
            vec![Dice::new(6, 1), Dice::new(3, 1)],
            1
        );
        assert_eq!(dn, expected);
    }

    #[test]
    fn add_assign_dice() {
        let mut dn = DiceNotation::new(
            vec![Dice::new(6, 2)], vec![Dice::new(6, 1)], 2
        );
        dn += Dice::new(8, 1);
        assert_eq!(dn.additions, vec![Dice::new(6, 2), Dice::new(8, 1)]);
    }

    #[test]
    fn sub_assign_dice() {
        let mut dn = DiceNotation::new(
            vec![Dice::new(6, 2)], vec![Dice::new(6, 1)], 2
        );
        dn -= Dice::new(8, 1);
        assert_eq!(dn.subtractions, vec![Dice::new(6, 1), Dice::new(8, 1)]);
    }

    #[test]
    fn add_assign_i32() {
        let mut dn = DiceNotation::new(
            vec![Dice::new(6, 2)], vec![Dice::new(6, 1)], 2
        );
        dn += 2;
        assert_eq!(dn.constant, 4);
    }

    #[test]
    fn sub_assign_i32() {
        let mut dn = DiceNotation::new(
            vec![Dice::new(6, 2)], vec![Dice::new(6, 1)], 2
        );
        dn -= 2;
        assert_eq!(dn.constant, 0);
    }

    #[test_case("d6" => it eq Ok(
        DiceNotation { 
            additions: vec![Dice::new(6, 1)],
            subtractions: vec![],
            constant: 0
        }
    ) ; "d6")]
    #[test_case("2d6" => it eq Ok(
        DiceNotation { 
            additions: vec![Dice::new(6, 2)], 
            subtractions: vec![], 
            constant: 0
        }
    ) ; "2d6")]
    #[test_case("2d6 - d6 + 2" => it eq Ok(
        DiceNotation { 
            additions: vec![Dice::new(6, 2)], 
            subtractions: vec![Dice::new(6, 1)], 
            constant: 2
        }
    ) ; "2d6 - d6 + 2")]
    #[test_case("4d8 - 3" => it eq Ok(
        DiceNotation { 
            additions: vec![Dice::new(8, 4)], 
            subtractions: vec![], 
            constant: -3
        }
    ) ; "4d8 - 3")]
    #[test_case("invalid" => matches Err(_) ; "invalid")]
    fn try_from_str(data: &str) -> Result<DiceNotation, &str> {
        DiceNotation::try_from(data)
    }

    #[test_case(DiceNotation::try_from("d6").unwrap() => it eq 1 ; "d6")]
    #[test_case(DiceNotation::try_from("2d6").unwrap() => it eq 2 ; "2d6")]
    #[test_case(DiceNotation::try_from("2d6 - d6 + 2").unwrap() => it eq 0 ; "2d6 - d6 + 2")]
    #[test_case(DiceNotation::try_from("4d8 - 3").unwrap() => it eq 1 ; "4d8 - 3")]
    fn min(dice_notation: DiceNotation) -> u32 {
        dice_notation.min()
    }

    #[test_case(DiceNotation::try_from("d6").unwrap() => it eq 6 ; "d6")]
    #[test_case(DiceNotation::try_from("2d6").unwrap() => it eq 12 ; "2d6")]
    #[test_case(DiceNotation::try_from("2d6 - d6 + 2").unwrap() => it eq 13 ; "2d6 - d6 + 2")]
    #[test_case(DiceNotation::try_from("4d8 - 3").unwrap() => it eq 29 ; "4d8 - 3")]
    fn max(dice_notation: DiceNotation) -> u32 {
        dice_notation.max()
    }

    #[test_case(DiceNotation::try_from("d6").unwrap() => it eq 3.5 ; "d6")]
    #[test_case(DiceNotation::try_from("2d6").unwrap() => it eq 7.0 ; "2d6")]
    #[test_case(DiceNotation::try_from("2d6 - d6 + 2").unwrap() => it eq 5.5 ; "2d6 - d6 + 2")]
    #[test_case(DiceNotation::try_from("4d8 - 3").unwrap() => it eq 15.0 ; "4d8 - 3")]
    fn average(dice_notation: DiceNotation) -> f32 {
        dice_notation.average()
    }
}
