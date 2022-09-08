use aos_statshammer::UnitComparator;
use float_eq::assert_float_eq;

mod data;

static NUM_SIMULATIONS: u32 = 10_000;
static PRECISION: f32 = 0.5;

macro_rules! assert_averages_similar {
    ($results: expr, [$a0: expr, $a1: expr, $a2: expr, $a3: expr, $a4: expr, $a5: expr, $a6: expr]) => {
        assert_eq!($results.len(), 7);
        assert_float_eq!($results[0].average, $a0, abs <= PRECISION);
        assert_float_eq!($results[1].average, $a1, abs <= PRECISION);
        assert_float_eq!($results[2].average, $a2, abs <= PRECISION);
        assert_float_eq!($results[3].average, $a3, abs <= PRECISION);
        assert_float_eq!($results[4].average, $a4, abs <= PRECISION);
        assert_float_eq!($results[5].average, $a5, abs <= PRECISION);
        assert_float_eq!($results[6].average, $a6, abs <= PRECISION);
    };
}

#[test]
fn compare_simulated_damage() {
    let units = [data::gotrek()];
    let comparator = UnitComparator::new(&units, None);
    let output = comparator.compare_simulated_damage(NUM_SIMULATIONS);
    assert_eq!(output.len(), 1);
    assert_averages_similar!(
        output[0].results,
        [9.407, 11.778, 14.148, 16.519, 18.889, 18.889, 18.889]
    );
}
