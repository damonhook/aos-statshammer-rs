use aos_statshammer_core::{processors::*, testutils::weapons};
use float_eq::assert_float_eq;

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

macro_rules! assert_average_damage_eq {
    ($weapon: expr, $expected: expr) => {
        assert_processor_results_eq!(
            AverageDamageProcessor::new(&$weapon).average_damage(),
            $expected
        );
    };
}

#[test]
fn average_damage_gotrek() {
    assert_average_damage_eq!(
        weapons::gotrek::zangrom_thaz(),
        processor_results!(9.407, 11.778, 14.148, 16.519, 18.889, 18.889, 18.889)
    );
}

#[test]
fn average_hearthguard_berserkers() {
    assert_average_damage_eq!(
        weapons::hearthguard_berserkers::broadaxes(),
        processor_results!(6.074, 12.148, 18.222, 24.296, 30.370, 36.444, 36.444)
    );
}

#[test]
fn average_chainrasp_horde() {
    assert_average_damage_eq!(
        weapons::chainrasp_horde::malignant_weapon(),
        processor_results!(0.875, 0.875, 1.75, 2.625, 3.5, 4.375, 5.25)
    );
}

#[test]
fn average_mortek_guard() {
    assert_average_damage_eq!(
        weapons::mortek_guard::nadirite_blade(),
        processor_results!(1.319, 2.639, 3.958, 5.278, 6.597, 7.917, 7.917)
    );
    assert_average_damage_eq!(
        weapons::mortek_guard::soulcleaver_greatblade(),
        processor_results!(0.148, 0.296, 0.444, 0.593, 0.741, 0.889, 0.889)
    );
}
