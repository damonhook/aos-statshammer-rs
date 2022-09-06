use aos_statshammer::UnitComparator;

mod data;

static NUM_SIMULATIONS: u32 = 10_000;

#[test]
fn compare_simulated_damage() {
    let units = [data::gotrek()];
    let comparator = UnitComparator::new(&units, None);
    let output = comparator.simulate_damage(NUM_SIMULATIONS);
    let means = output.iter().map(|o| o.mean()).collect::<Vec<_>>();
    assert_eq!(output.len(), 1);
    assert_eq!(means.len(), 1);
}
