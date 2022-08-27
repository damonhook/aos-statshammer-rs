use aos_statshammer::{AverageComparisonResult, UnitComparator};
use aos_statshammer_core::processors::{ProcessorResults, SaveResult};
use float_eq::*;

mod data;

static PRECISION: f32 = 0.000_5; // Approximately 3 decimal places

macro_rules! processor_results {
    ($r1: expr, $r2: expr, $r3: expr, $r4: expr, $r5: expr, $r6: expr, $r7: expr) => {
        ProcessorResults::from([
            SaveResult::new(1, $r1),
            SaveResult::new(2, $r2),
            SaveResult::new(3, $r3),
            SaveResult::new(4, $r4),
            SaveResult::new(5, $r5),
            SaveResult::new(6, $r6),
            SaveResult::new(7, $r7),
        ])
    };
}

macro_rules! assert_processor_results_eq {
    ($left: expr, $right: expr) => {
        assert!(matches!($left, ProcessorResults { .. }));
        assert_eq!($left.save_results.len(), $right.save_results.len());
        for (index, right_result) in $right.save_results.iter().enumerate() {
            assert_eq!($left.save_results[index].save, right_result.save);
            assert_float_eq!(
                $left.save_results[index].value,
                right_result.value,
                abs <= PRECISION
            );
        }
    };
}

macro_rules! assert_average_comparison_result_eq {
    ($left: expr, $right: expr) => {
        assert!(matches!($left, AverageComparisonResult { .. }));
        assert_eq!($left.name, $right.name);
        assert_processor_results_eq!($left.averages, $right.averages);
        assert_eq!($left.max, $right.max);
    };
}

#[test]
fn compare_average_damage() {
    let units = [data::gotrek(), data::chainrasp_horde()];
    let comparator = UnitComparator::new(&units);
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
