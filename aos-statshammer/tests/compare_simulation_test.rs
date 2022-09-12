use aos_statshammer::UnitComparator;
use derive_new::new;
use float_eq::assert_float_eq;
use test_case::test_case;

mod data;

static NUM_SIMULATIONS: u32 = 20_000;
static PRECISION: f32 = 0.5;

#[derive(new)]
struct Expected {
    average: f32,
    max: u32,
}

#[test_case(1, &[Expected::new(9.407, 54), Expected::new(0.875, 21)] ; "1+ Save")]
#[test_case(2, &[Expected::new(11.778, 54), Expected::new(0.875, 21)] ; "2+ Save")]
#[test_case(3, &[Expected::new(14.148, 54), Expected::new(1.75, 21)] ; "3+ Save")]
#[test_case(4, &[Expected::new(16.519, 54), Expected::new(2.625, 21)] ; "4+ Save")]
#[test_case(5, &[Expected::new(18.889, 54), Expected::new(3.5, 21)] ; "5+ Save")]
#[test_case(6, &[Expected::new(18.889, 54), Expected::new(4.375, 21)] ; "6+ Save")]
#[test_case(7, &[Expected::new(18.889, 54), Expected::new(5.25, 21)] ; "7+ Save")]
fn compare_simulated_damage(save: u32, expected: &[Expected]) {
    let units = [data::gotrek(), data::chainrasp_horde()];
    let comparator = UnitComparator::new(&units, None);
    let output = comparator.compare_simulated_damage(save, NUM_SIMULATIONS);

    assert_eq!(output.len(), expected.len());
    for idx in 0..output.len() {
        assert_float_eq!(output[idx].average, expected[idx].average, abs <= PRECISION);
        assert_eq!(output[idx].max, expected[idx].max);
    }
}
