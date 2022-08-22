/// Enum representing the different characteristics that a `Weapon` can roll for
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum RollCharacteristic {
    Hit,
    Wound,
}

/// Enum representing the different characteristics that a `Weapon` can have
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Characteristic {
    Attacks,
    Roll(RollCharacteristic),
    Rend,
    Damage,
}
