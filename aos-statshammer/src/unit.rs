use aos_statshammer_core::{
    processors::{
        AverageDamageProcessor, MaxDamageProcessor, ProcessorResults, SimulatedDamageProcessor,
    },
    DiceRoller, Opponent, Weapon,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnitSimulationResults {
    pub buckets: HashMap<u32, u32>,
}
impl UnitSimulationResults {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn incr(&mut self, damage: u32) {
        if let Some(v) = self.buckets.get_mut(&damage) {
            *v += 1;
        } else {
            self.buckets.insert(damage, 1);
        }
    }

    pub fn mean(&self) -> f32 {
        let mut running_total: u32 = 0;
        let mut running_count: u32 = 0;
        for (k, v) in self.buckets.iter() {
            running_count += v;
            running_total += k * v;
        }
        running_total as f32 / running_count as f32
    }
}

impl Default for UnitSimulationResults {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn roll_damage(&self, save: u32, opponent: Option<&Opponent>) -> u32 {
        self.weapons.iter().fold(0, |acc, weapon| {
            let mut processor = SimulatedDamageProcessor::new(weapon, save);
            if let Some(o) = opponent {
                processor.opponent(o);
            }
            let roller = DiceRoller::default();
            acc + processor.simulate_damage(&roller)
        })
    }

    pub fn simulate_damage(
        &self,
        save: u32,
        opponent: Option<&Opponent>,
        num_simulations: u32,
    ) -> UnitSimulationResults {
        let mut results = UnitSimulationResults::new();
        for _ in 0..num_simulations {
            results.incr(self.roll_damage(save, opponent));
        }
        results
    }
}
