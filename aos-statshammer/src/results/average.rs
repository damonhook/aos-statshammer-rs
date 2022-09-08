use aos_statshammer_core::processors::ProcessorResults;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AverageComparisonResult {
    pub name: String,
    pub averages: ProcessorResults,
    pub max: u32,
}
