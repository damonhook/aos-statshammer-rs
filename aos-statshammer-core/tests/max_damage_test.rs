mod data;
use aos_statshammer_core::processors::*;

macro_rules! assert_max_damage_eq {
    ($weapon: expr, $expected: expr) => {
        assert_eq!(MaxDamageProcessor::new(&$weapon).max_damage(), $expected);
    };
}

#[test]
fn average_damage_gotrek_axe() {
    assert_max_damage_eq!(data::gotrek_axe(), 54);
}

#[test]
fn average_hearthguard_berserkers_broadaxes() {
    assert_max_damage_eq!(data::hearthguard_berserkers_broadaxes(), 82);
}

#[test]
fn average_chainrasp_horde() {
    assert_max_damage_eq!(data::chainrasp_horde(), 21);
}
