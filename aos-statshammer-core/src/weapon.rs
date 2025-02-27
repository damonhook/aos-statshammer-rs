use crate::RerollType;
use crate::characteristics::*;
use crate::dice::D6;
use crate::target::Target;
use derive_builder::Builder;

pub use crate::characteristics::ExplodingAbility;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Attacks {
    pub value: u8,
    pub bonus: i16,
}
impl_characteristic!(Attacks, value, bonus, +);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]

pub struct Hit {
    pub value: u8,
    pub bonus: i16,
    pub reroll: Option<RerollType>,
    pub exploding: Option<ExplodingAbility>,
}
impl_characteristic!(Hit, value, bonus, -);
impl_reroll!(Hit, reroll);
impl_exploding!(Hit, exploding);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Wound {
    pub value: u8,
    pub bonus: i16,
    pub reroll: Option<RerollType>,
    pub exploding: Option<ExplodingAbility>,
}
impl_characteristic!(Wound, value, bonus, -);
impl_reroll!(Wound, reroll);
impl_exploding!(Wound, exploding);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Rend {
    pub value: u8,
    pub bonus: i16,
}
impl_characteristic!(Rend, value, bonus, +);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Damage {
    pub value: u8,
    pub bonus: i16,
}
impl_characteristic!(Damage, value, bonus, +);

#[derive(Debug, Builder, Eq, PartialEq)]
pub struct Weapon {
    #[builder(setter(into))]
    pub attacks: Attacks,
    #[builder(setter(into))]
    pub hit: Hit,
    #[builder(setter(into))]
    pub wound: Wound,
    #[builder(setter(into))]
    pub rend: Rend,
    #[builder(setter(into))]
    pub damage: Damage,
}

impl Weapon {
    pub fn average_damage(&self, target: &Target) -> f64 {
        let average_attacks = self.attacks.modified() as f64;

        let mut average_hit_rolls =
            average_attacks + (average_attacks * self.hit.reroll_probability());
        average_hit_rolls += average_attacks * self.hit.extra_probability();
        let average_hits = average_hit_rolls * D6.probability(self.hit.modified());

        let mut average_wound_rolls =
            average_hits + (average_hits * self.wound.reroll_probability());
        average_wound_rolls += average_hits * self.wound.extra_probability();
        let average_wounds = average_wound_rolls * D6.probability(self.wound.modified());

        let average_successful = average_wounds * target.average_unsaved(self.rend.modified());
        average_successful * self.damage.modified() as f64
    }
}
