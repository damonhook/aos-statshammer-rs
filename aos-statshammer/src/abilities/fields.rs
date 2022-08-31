#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NumberField {
    pub field_id: String,
    pub display_name: String,
    pub min_value: Option<u32>,
    pub max_value: Option<u32>,
    pub default: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DiceNotationField {
    pub field_id: String,
    pub display_name: String,
    pub default: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Choice {
    pub value: String,
    pub display_value: String,
}
impl Choice {
    pub fn new(value: &str, display_value: &str) -> Self {
        Self {
            value: value.into(),
            display_value: display_value.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ChoiceField {
    pub field_id: String,
    pub display_name: String,
    pub choices: Vec<Choice>,
    pub default: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RollField {
    pub field_id: String,
    pub display_name: String,
    pub min_value: u32,
    pub default: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BooleanField {
    pub field_id: String,
    pub display_name: String,
    pub default: Option<bool>,
}

#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum Field {
    Number(NumberField),
    DiceNotation(DiceNotationField),
    Choice(ChoiceField),
    Roll(RollField),
    Boolean(BooleanField),
}
