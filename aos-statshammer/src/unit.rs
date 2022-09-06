use aos_statshammer_core::{
    processors::{
        AverageDamageProcessor, MaxDamageProcessor, ProcessorResults, SimulatedDamageProcessor,
    },
    Opponent, Weapon,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Defines a single Age of Sigmar unit which can contain any number of [Weapon] structs.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// Calculate the average damage for all of the [Weapon] structs that belong to this unit.
    pub fn average_damage(&self, opponent: Option<&Opponent>) -> ProcessorResults {
        let mut results = ProcessorResults::new();
        for weapon in self.weapons.iter() {
            let mut processor = AverageDamageProcessor::new(weapon);
            if let Some(o) = opponent {
                processor.opponent(o);
            }
            let weapon_results = processor.average_damage();
            results.merge(weapon_results);
        }
        results
    }

    /// Calculate the maximum damage for all of the [Weapon] structs that belong to this unit.
    pub fn max_damage(&self) -> u32 {
        self.weapons.iter().fold(0, |acc, weapon| {
            acc + MaxDamageProcessor::new(weapon).max_damage()
        })
    }

    pub fn simulate_damage(&self, save: u32, opponent: Option<&Opponent>) -> u32 {
        self.weapons.iter().fold(0, |acc, weapon| {
            let mut processor = SimulatedDamageProcessor::new(weapon, save);
            if let Some(o) = opponent {
                processor.opponent(o);
            }
            acc + processor.simulate_damage()
        })
    }
}
