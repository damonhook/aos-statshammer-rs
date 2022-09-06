#![allow(dead_code)]
use aos_statshammer::Unit;
use aos_statshammer_core::testutils;

pub fn gotrek() -> Unit {
    Unit::new("Gotrek", vec![testutils::weapons::gotrek::zangrom_thaz()])
}

pub fn chainrasp_horde() -> Unit {
    Unit::new(
        "Chainrasp Horde",
        vec![testutils::weapons::chainrasp_horde::malignant_weapon()],
    )
}
