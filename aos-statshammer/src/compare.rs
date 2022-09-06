use std::collections::HashMap;

use crate::Unit;
use aos_statshammer_core::{processors::ProcessorResults, Opponent};
#[cfg(feature = "serde")]
use serde::Serialize;

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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AverageComparisonResult {
    pub name: String,
    pub averages: ProcessorResults,
    pub max: u32,
}

/// Used to run various comparisons between multiple [Units](Unit).
pub struct UnitComparator<'a> {
    units: &'a [Unit],
    opponent: Option<&'a Opponent>,
}
impl<'a> UnitComparator<'a> {
    pub fn new(units: &'a [Unit], opponent: Option<&'a Opponent>) -> Self {
        Self { units, opponent }
    }

    pub fn compare_average_damage(&self) -> Vec<AverageComparisonResult> {
        self.units
            .iter()
            .map(|u| AverageComparisonResult {
                name: u.name.clone(),
                averages: u.average_damage(self.opponent),
                max: u.max_damage(),
            })
            .collect()
    }

    pub fn simulate_damage(&self, num_simulations: u32) -> Vec<UnitSimulationResults> {
        self.units
            .iter()
            .map(|unit| {
                let mut results = UnitSimulationResults::new();
                for _ in 0..num_simulations {
                    results.incr(unit.simulate_damage(4, self.opponent));
                }
                results
            })
            .collect()
    }
}
