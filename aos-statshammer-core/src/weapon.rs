use crate::rollable::DiceNotation;

#[derive(Debug, PartialEq)]
pub struct Weapon {
    pub models: u32,
    pub attacks: DiceNotation,
    pub to_hit: u32,
    pub to_wound: u32,
    pub rend: u32,
    pub damage: DiceNotation,
}

impl Weapon {
    pub fn new(
        models: u32,
        attacks: DiceNotation,
        to_hit: u32,
        to_wound: u32,
        rend: u32,
        damage: DiceNotation,
    ) -> Self {
        Self {
            models,
            attacks,
            to_hit,
            to_wound,
            rend,
            damage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let output = Weapon::new(10, DiceNotation::from(2), 3, 4, 1, DiceNotation::from(3));
        let expected = Weapon {
            models: 10,
            attacks: DiceNotation::from(2),
            to_hit: 3,
            to_wound: 4,
            rend: 1,
            damage: DiceNotation::from(3),
        };
        assert_eq!(output, expected);
    }
}
