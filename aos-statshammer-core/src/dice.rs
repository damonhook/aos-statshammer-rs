pub(crate) struct Dice {
    sides: usize,
}

impl Dice {
    #[allow(dead_code)]
    pub fn new(sides: usize) -> Self {
        Self { sides }
    }

    pub fn probability(&self, target: u8) -> f64 {
        let upper_bounds = (self.sides as f64) + 1.0;
        let target: f64 = target.into();
        if target > upper_bounds {
            0.0
        } else {
            ((upper_bounds - target) / self.sides as f64).clamp(0.0, 1.0)
        }
    }

    pub fn inverse_probability(&self, target: u8) -> f64 {
        1.0 - self.probability(target)
    }
}

pub(crate) const D6: Dice = Dice {sides: 6};