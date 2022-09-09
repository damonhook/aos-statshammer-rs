use aos_statshammer::UnitComparator;
use float_eq::assert_float_eq;
use test_case::test_case;

mod data;

static NUM_SIMULATIONS: u32 = 100_000;
static PRECISION: f32 = 0.5;

#[test_case(1, 9.407 ; "1+ Save")]
#[test_case(2, 11.778 ; "2+ Save")]
#[test_case(3, 14.148 ; "3+ Save")]
#[test_case(4, 16.519 ; "4+ Save")]
#[test_case(5, 18.889 ; "5+ Save")]
#[test_case(6, 18.889 ; "6+ Save")]
#[test_case(7, 18.889 ; "7+ Save")]
fn compare_simulated_damage(save: u32, expected_average: f32) {
    let units = [data::gotrek()];
    let comparator = UnitComparator::new(&units, None);
    let output = comparator.compare_simulated_damage(save, NUM_SIMULATIONS);
    assert_eq!(output.len(), 1);
    assert_float_eq!(output[0].average, expected_average, abs <= PRECISION);
    assert_eq!(output[0].max, 54);
}
