use std::{
    cmp,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num_traits::Zero;

pub trait Max {
    fn max(self, other: Self) -> Self;
}

impl Max for i32 {
    fn max(self, other: i32) -> i32 {
        cmp::max(self, other)
    }
}

impl Max for f32 {
    fn max(self, other: f32) -> f32 {
        f32::max(self, other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollTarget<T> {
    initial: T,
    pub modifier: T,
    min_value: Option<T>,
}

impl<T> RollTarget<T>
where
    T: Sub<T, Output = T> + Max + Copy,
{
    pub fn new(initial: T, modifier: T, min_value: Option<T>) -> Self {
        Self {
            initial,
            modifier,
            min_value,
        }
    }

    /// Get the unmodified roll target constrained by the given bounds
    pub fn unmodified(&self) -> T {
        self.with_min(self.initial)
    }

    /// Get the modified roll target constrained by the given bounds
    pub fn modified(&self) -> T {
        self.with_min(self.initial - self.modifier)
    }

    /// Get either the modified or unmodified roll target value constrained by the given bounds
    pub fn value(&self, unmodified: bool) -> T {
        if unmodified {
            self.unmodified()
        } else {
            self.modified()
        }
    }

    fn with_min(&self, value: T) -> T {
        match self.min_value {
            Some(m) => value.max(m),
            _ => value,
        }
    }

    pub fn clone_with_initial(&self, initial: T) -> Self {
        Self {
            initial,
            modifier: self.modifier,
            min_value: self.min_value,
        }
    }
}

impl<T> From<T> for RollTarget<T>
where
    T: Zero + Max + Copy,
{
    fn from(d: T) -> Self {
        Self {
            initial: d,
            modifier: T::zero(),
            min_value: None,
        }
    }
}

impl<T> Add<T> for RollTarget<T>
where
    T: Add<T, Output = T> + Max + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            initial: self.initial,
            modifier: self.modifier + rhs,
            min_value: self.min_value,
        }
    }
}

impl<T> AddAssign<T> for RollTarget<T>
where
    T: Add<T, Output = T> + AddAssign<T> + Max + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl<T> Sub<T> for RollTarget<T>
where
    T: Sub<T, Output = T> + Max + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            initial: self.initial,
            modifier: self.modifier - rhs,
            min_value: self.min_value,
        }
    }
}

impl<T> SubAssign<T> for RollTarget<T>
where
    T: Sub<T, Output = T> + SubAssign<T> + Max + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn roll_target_new() {
        let output: RollTarget<f32> = RollTarget::new(3., 2., Some(2.));
        let expected: RollTarget<f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value: Some(2.),
        };
        assert_eq!(output, expected);
    }

    #[test_case(None => 3. ; "No Bounds")]
    #[test_case(Some(4.) => 4. ; "Min Bounded")]
    fn roll_target_unmodified(min_value: Option<f32>) -> f32 {
        let target: RollTarget<f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value,
        };
        target.unmodified()
    }

    #[test_case(None => 1. ; "No Bounds")]
    #[test_case(Some(4.) => 4. ; "Min Bounded")]
    fn roll_target_modified(min_value: Option<f32>) -> f32 {
        let target: RollTarget<f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value,
        };
        target.modified()
    }

    #[test]
    fn roll_target_from() {
        let output: RollTarget<f32> = RollTarget::from(2.);
        let expected: RollTarget<f32> = RollTarget {
            initial: 2.,
            modifier: 0.,
            min_value: None,
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn roll_target_add() {
        let a: RollTarget<f32> = RollTarget {
            initial: 2.,
            modifier: 1.,
            min_value: None,
        };
        assert_eq!(a.modifier, 1.); // Precondition

        let b: RollTarget<f32> = a + 2.;
        assert_eq!(b.modifier, 3.); // New RollTarget should have the new modifier value
        assert_eq!(a.modifier, 1.); // Check no mutation
    }

    #[test]
    fn roll_target_add_assign() {
        let mut a: RollTarget<f32> = RollTarget {
            initial: 2.,
            modifier: 1.,
            min_value: None,
        };
        assert_eq!(a.modifier, 1.); // Precondition

        a += 2.;
        assert_eq!(a.modifier, 3.); // Check mutation
    }
}
