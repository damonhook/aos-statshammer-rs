use num_traits::Zero;
use std::{
    cmp,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub trait RollTargetValue<T> {
    /// Get the unmodified roll target constrained by the given bounds
    fn unmodified(&self) -> T;

    /// Get the modified roll target constrained by the given bounds
    fn modified(&self) -> T;

    /// Get either the modified or unmodified roll target value constrained by the given bounds
    fn value(&self, unmodified: bool) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RollTarget<T, U> {
    initial: T,
    pub modifier: U,
    min_value: Option<T>,
}

impl<T, U> RollTarget<T, U>
where
    T: Copy,
    U: Copy,
{
    pub fn new(initial: T, modifier: U, min_value: Option<T>) -> Self {
        Self {
            initial,
            modifier,
            min_value,
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

impl RollTargetValue<f32> for RollTarget<f32, f32> {
    fn unmodified(&self) -> f32 {
        self.value(true)
    }

    fn modified(&self) -> f32 {
        self.value(false)
    }

    fn value(&self, unmodified: bool) -> f32 {
        let v = if unmodified {
            self.initial
        } else {
            self.initial - self.modifier
        };
        self.min_value.map_or(v, |m| v.max(m))
    }
}

impl RollTargetValue<u32> for RollTarget<u32, i32> {
    fn unmodified(&self) -> u32 {
        self.value(true)
    }

    fn modified(&self) -> u32 {
        self.value(false)
    }

    fn value(&self, unmodified: bool) -> u32 {
        let v: u32 = if unmodified {
            self.initial
        } else if self.modifier < (self.initial as i32) {
            (self.initial as i32 - self.modifier) as u32
        } else {
            0
        };
        self.min_value.map_or(v, |m| cmp::max(v, m))
    }
}

impl<T, U> From<T> for RollTarget<T, U>
where
    U: Zero,
{
    fn from(d: T) -> Self {
        Self {
            initial: d,
            modifier: U::zero(),
            min_value: None,
        }
    }
}

impl<T, U> AddAssign<U> for RollTarget<T, U>
where
    U: Add<U, Output = U> + AddAssign<U> + Copy,
{
    fn add_assign(&mut self, rhs: U) {
        self.modifier += rhs;
    }
}

impl<T, U> SubAssign<U> for RollTarget<T, U>
where
    U: Sub<U, Output = U> + SubAssign<U> + Copy,
{
    fn sub_assign(&mut self, rhs: U) {
        self.modifier -= rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn roll_target_new() {
        let output: RollTarget<f32, f32> = RollTarget::new(3., 2., Some(2.));
        let expected: RollTarget<f32, f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value: Some(2.),
        };
        assert_eq!(output, expected);
    }

    #[test_case(None => 3. ; "No Bounds")]
    #[test_case(Some(4.) => 4. ; "Min Bounded")]
    fn roll_target_unmodified(min_value: Option<f32>) -> f32 {
        let target: RollTarget<f32, f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value,
        };
        target.unmodified()
    }

    #[test_case(None => 1. ; "No Bounds")]
    #[test_case(Some(4.) => 4. ; "Min Bounded")]
    fn roll_target_modified(min_value: Option<f32>) -> f32 {
        let target: RollTarget<f32, f32> = RollTarget {
            initial: 3.,
            modifier: 2.,
            min_value,
        };
        target.modified()
    }

    #[test]
    fn roll_target_from() {
        let output = RollTarget::from(2.);
        let expected = RollTarget::<f32, f32> {
            initial: 2.,
            modifier: 0.,
            min_value: None,
        };
        assert_eq!(output, expected);
    }

    #[test]
    fn roll_target_add_assign() {
        let mut a = RollTarget::new(2., 1., None);
        assert_eq!(a.modifier, 1.); // Precondition

        a += 2.;
        assert_eq!(a.modifier, 3.); // Check mutation
    }
}
