use aos_statshammer_core::{processors::*, testutils::weapons};

macro_rules! assert_max_damage_eq {
    ($weapon: expr, $expected: expr) => {
        assert_eq!(MaxDamageProcessor::new(&$weapon).max_damage(), $expected);
    };
}

#[test]
fn max_damage_gotrek() {
    assert_max_damage_eq!(weapons::gotrek::zangrom_thaz(), 54);
}

#[test]
fn max_hearthguard_berserkers() {
    assert_max_damage_eq!(weapons::hearthguard_berserkers::broadaxes(), 82);
}

#[test]
fn max_chainrasp_horde() {
    assert_max_damage_eq!(weapons::chainrasp_horde::malignant_weapon(), 21);
}

#[test]
fn max_mortek_guard() {
    assert_max_damage_eq!(weapons::mortek_guard::nadirite_blade(), 38);
    assert_max_damage_eq!(weapons::mortek_guard::soulcleaver_greatblade(), 2);
}
