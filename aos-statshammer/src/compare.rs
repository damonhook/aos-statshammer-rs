use crate::{
    results::{
        average::AverageComparisonResult,
        simulation::{Buckets, SimulatedUnitResult},
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

    pub fn compare_simulated_damage(
        &self,
        save: u32,
        num_simulations: u32,
    ) -> Vec<SimulatedUnitResult> {
        self.units
            .iter()
            .map(|unit| {
                let result = unit.simulate_damage(save, self.opponent, num_simulations);
                SimulatedUnitResult {
                    name: unit.name.clone(),
                    results: Buckets::from(&result.buckets),
                    average: result.mean(),
                    max: unit.max_damage(),
                }
            })
            .collect()
    }
}
