use crate::{
    results::{
        average::AverageComparisonResult,
        simulation::{Buckets, SaveSimulatedResult, SimulatedUnitResult},
    },
    unit::Unit,
};
use aos_statshammer_core::Opponent;

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

    pub fn compare_simulated_damage(&self, num_simulations: u32) -> Vec<SimulatedUnitResult> {
        self.units
            .iter()
            .map(|unit| SimulatedUnitResult {
                name: unit.name.clone(),
                results: (1..=7)
                    .map(|save| {
                        let result = unit.simulate_damage(save, self.opponent, num_simulations);
                        SaveSimulatedResult {
                            save,
                            buckets: Buckets::from(&result.buckets),
                            average: result.mean(),
                        }
                    })
                    .collect(),
                max: unit.max_damage(),
            })
            .collect()
    }
}
