use aos_statshammer_core::{
    processors::{AverageDamageProcessor, MaxDamageProcessor, ProcessorResults},
    Weapon,
};

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    weapons: Vec<Weapon>,
}

impl Unit {
    pub fn new(name: &str, weapons: Vec<Weapon>) -> Self {
        Self {
            name: name.into(),
            weapons,
        }
    }

    pub fn average_damage(&self) -> ProcessorResults {
        let mut results = ProcessorResults::new();
        for weapon in self.weapons.iter() {
            let weapon_results = AverageDamageProcessor::new(weapon).average_damage();
            results.merge(weapon_results);
        }
        results
    }

    pub fn max_damage(&self) -> u32 {
        self.weapons.iter().fold(0, |acc, weapon| {
            acc + MaxDamageProcessor::new(weapon).max_damage()
        })
    }
}
