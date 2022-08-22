mod dice;
pub use dice::*;

mod dice_notation;
pub use dice_notation::*;

pub trait Rollable {
    fn min(&self) -> u32;
    fn max(&self) -> u32;
    fn average(&self) -> f32;
    fn roll(&self) -> u32;
}
