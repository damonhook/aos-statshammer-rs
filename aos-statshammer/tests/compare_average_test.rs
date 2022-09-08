use aos_statshammer::{average::AverageComparisonResult, UnitComparator};
use aos_statshammer_core::processors::{ProcessorResults, SaveResult};
use aos_statshammer_core::{assert_processor_results_eq, processor_results};
use float_eq::*;

mod data;

static PRECISION: f32 = 0.000_5; // Approximately 3 decimal places

macro_rules! assert_average_comparison_result_eq {
    ($left: expr, $right: expr) => {
        assert!(matches!($left, AverageComparisonResult { .. }));
        assert_eq!($left.name, $right.name);
        assert_processor_results_eq!($left.averages, $right.averages, PRECISION);
        assert_eq!($left.max, $right.max);
    };
}

#[test]
fn compare_average_damage() {
    let units = [data::gotrek(), data::chainrasp_horde()];
    let comparator = UnitComparator::new(&units, None);
    let output = comparator.compare_average_damage();

    let expected = vec![
        AverageComparisonResult {
            name: "Gotrek".into(),
            averages: processor_results!(9.407, 11.778, 14.148, 16.519, 18.889, 18.889, 18.889),
            max: 54,
        },
        AverageComparisonResult {
            name: "Chainrasp Horde".into(),
            averages: processor_results!(0.875, 0.875, 1.75, 2.625, 3.5, 4.375, 5.25),
            max: 21,
        },
    ];
    assert_eq!(output.len(), expected.len());
    for (index, result) in output.iter().enumerate() {
        assert_average_comparison_result_eq!(result, expected[index]);
    }
}
