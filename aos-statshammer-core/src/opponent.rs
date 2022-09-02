use crate::abilities::opponent::{OpponentAbility, SaveReroll};

#[derive(Debug)]
pub struct Opponent {
    pub abilities: Vec<OpponentAbility>,
}
impl Opponent {
    pub fn new(abilities: Vec<OpponentAbility>) -> Self {
        Self { abilities }
    }

    pub fn reroll_ability(&self) -> Option<&SaveReroll> {
        self.abilities
            .iter()
            .filter_map(|ability| match ability {
                OpponentAbility::SaveReroll(a) => Some(a),
                _ => None,
            })
            .max_by_key(|a| a.reroll_type)
    }
}
