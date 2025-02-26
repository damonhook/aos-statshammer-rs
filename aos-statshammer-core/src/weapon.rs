use crate::characteristics::{RollTarget, Value};
use crate::target::Target;
use derive_builder::Builder;

pub type Attacks = Value;
pub type Hit = RollTarget;
pub type Wound = RollTarget;
pub type Rend = Value;
pub type Damage = Value;

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
        let average_hits = self.hit.average(self.attacks.modified().into());
        let average_wounds = self.wound.average(average_hits);
        let average_successful = average_wounds * target.average_unsaved(self.rend.modified());
        average_successful * self.damage.modified() as f64
    }
}