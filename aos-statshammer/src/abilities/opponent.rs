use super::RerollType;
use super::{fields::*, AbilityDefinition};
pub use aos_statshammer_core::abilities::opponent::*;

impl AbilityDefinition for SaveReroll {
    fn name() -> String {
        "Reroll".into()
    }

    fn description() -> String {
        "Reroll {reroll_type} for {characteristic}".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::Choice(ChoiceField {
            field_id: "reroll_type".into(),
            display_name: "Reroll Type".into(),
            choices: vec![
                Choice::new(&RerollType::Any.to_string(), "Any Roll"),
                Choice::new(&RerollType::Failed.to_string(), "Failed Rolls"),
                Choice::new(&RerollType::Ones.to_string(), "Rolls of 1"),
            ],
            default: None,
        })]
    }
}

impl AbilityDefinition for SaveBonus {
    fn name() -> String {
        "Bonus".into()
    }

    fn description() -> String {
        "Add {value} to save".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::DiceNotation(DiceNotationField {
            field_id: "value".into(),
            display_name: "Value".into(),
            default: None,
        })]
    }
}

impl AbilityDefinition for Ward {
    fn name() -> String {
        "Ward Save".into()
    }

    fn description() -> String {
        "Ignore wounds and mortal wounds on a roll of {on}+".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::Roll(RollField {
            field_id: "on".into(),
            display_name: "On".into(),
            min_value: 2,
            default: None,
        })]
    }
}

impl AbilityDefinition for Ethereal {
    fn name() -> String {
        "Ethereal".into()
    }

    fn description() -> String {
        "Ignore modifiers to save (positive or negative) when making save rolls'".into()
    }

    fn fields() -> Vec<Field> {
        vec![]
    }
}
