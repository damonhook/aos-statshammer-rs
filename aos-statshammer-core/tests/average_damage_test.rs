mod data;
use aos_statshammer_core::processors::*;
use float_eq::*;

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

#[test]
fn average_damage_gotrek_axe() {
    let (weapon, abilities) = data::gotrek_axe();
    let processor = AverageDamageProcessor::new(&weapon, &abilities);
    let output = processor.average_damage();
    assert_processor_results_eq!(
        output,
        processor_results!(9.407, 11.778, 14.148, 16.519, 18.889, 18.889, 18.889)
    );
}

#[test]
fn average_hearthguard_berserkers_broadaxes() {
    let (weapon, abilities) = data::hearthguard_berserkers_broadaxes();
    let processor = AverageDamageProcessor::new(&weapon, &abilities);
    let output = processor.average_damage();
    assert_processor_results_eq!(
        output,
        processor_results!(6.074, 12.148, 18.222, 24.296, 30.370, 36.444, 36.444)
    );
}

#[test]
fn average_chainrasp_horde() {
    let (weapon, abilities) = data::chainrasp_horde();
    let processor = AverageDamageProcessor::new(&weapon, &abilities);
    let output = processor.average_damage();
    assert_processor_results_eq!(
        output,
        processor_results!(0.875, 0.875, 1.75, 2.625, 3.5, 4.375, 5.25)
    );
}
