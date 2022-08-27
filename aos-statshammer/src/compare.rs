use crate::Unit;
use aos_statshammer_core::processors::ProcessorResults;

#[derive(Debug)]
pub struct AverageComparisonResult {
    pub name: String,
    pub averages: ProcessorResults,
    pub max: u32,
}

pub struct UnitComparator<'a> {
    units: &'a [Unit],
}
impl<'a> UnitComparator<'a> {
    pub fn new(units: &'a [Unit]) -> Self {
        Self { units }
    }

    pub fn compare_average_damage(&self) -> Vec<AverageComparisonResult> {
        self.units
            .iter()
            .map(|u| AverageComparisonResult {
                name: u.name.clone(),
                averages: u.average_damage(),
                max: u.max_damage(),
            })
            .collect()
    }
}
