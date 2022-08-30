use serde::Serialize;

#[derive(Serialize)]
pub struct NumberField {
    pub field_id: String,
    pub display_name: String,
    pub min_value: Option<u32>,
    pub max_value: Option<u32>,
    pub default: Option<u32>,
}

#[derive(Serialize)]
pub struct DiceNotationField {
    pub field_id: String,
    pub display_name: String,
    pub default: Option<u32>,
}

#[derive(Serialize)]
pub struct Choice {
    value: String,
    display_value: String
}
impl Choice {
    pub fn new(value: &str, display_value: &str) -> Self {
        Self { value: value.into(), display_value: display_value.into() }
    }
}

#[derive(Serialize)]
pub struct ChoiceField {
    pub field_id: String,
    pub display_name: String,
    pub choices: Vec<Choice>,
    pub default: Option<String>,
}

#[derive(Serialize)]
pub struct RollField {
    pub field_id: String,
    pub display_name: String,
    pub min_value: u32,
    pub default: Option<u32>,
}

#[derive(Serialize)]
pub struct BooleanField {
    pub field_id: String,
    pub display_name: String,
    pub default: Option<bool>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Field {
    Number(NumberField),
    DiceNotation(DiceNotationField),
    Choice(ChoiceField),
    Roll(RollField),
    Boolean(BooleanField),
}
