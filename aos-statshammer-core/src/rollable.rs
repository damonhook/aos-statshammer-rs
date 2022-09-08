mod dice;
pub use dice::*;

mod dice_notation;
pub use dice_notation::*;

#[cfg(test)]
use mockall::automock;
use rand::Rng;

#[cfg_attr(test, automock)]
pub trait Roller {
    fn roll(&self, sides: u32) -> u32;
}

#[derive(Debug)]
pub struct DiceRoller;
impl DiceRoller {
    #[allow(unused)]
    fn new() -> Self {
        Self {}
    }
}
impl Default for DiceRoller {
    fn default() -> Self {
        Self {}
    }
}
impl Roller for DiceRoller {
    fn roll(&self, sides: u32) -> u32 {
        rand::thread_rng().gen_range(1..=sides)
    }
}

pub trait Rollable {
    fn min(&self) -> u32;
    fn max(&self) -> u32;
    fn average(&self) -> f32;
    fn roll<T: Roller>(&self, roller: &T) -> u32;
}
