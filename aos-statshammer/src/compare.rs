use crate::Unit;
use aos_statshammer_core::{processors::ProcessorResults, Opponent};
#[cfg(feature = "serde")]
use serde::Serialize;

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
}
