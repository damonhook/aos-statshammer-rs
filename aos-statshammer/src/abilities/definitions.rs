use super::fields::*;
use super::weapon::{Bonus, Exploding, LeaderExtraAttacks, MortalWounds, Reroll};
use super::RerollType;
use aos_statshammer_core::{RollCharacteristic as RollChar, ValueCharacteristic as ValChar};

pub trait AbilityDefinition {
    /// A display name for the Ability
    fn name() -> String;
    /// A description for the Ability with placeholders for where field values would be substituded
    fn description() -> String;
    /// A list of [Fields](Field) defining how the specific ability can configured
    fn fields() -> Vec<Field>;
}

macro_rules! characteristic_choices {
    () => {
        vec![
            Choice::new(&ValChar::Attacks.to_string(), "Attacks"),
            Choice::new(&RollChar::Hit.to_string(), "To Hit"),
            Choice::new(&RollChar::Wound.to_string(), "To Wound"),
            Choice::new(&ValChar::Rend.to_string(), "Rend"),
            Choice::new(&ValChar::Damage.to_string(), "Damage"),
        ]
    };
}

macro_rules! roll_characteristic_choices {
    () => {
        vec![
            Choice::new(&RollChar::Hit.to_string(), "To Hit"),
            Choice::new(&RollChar::Wound.to_string(), "To Wound"),
        ]
    };
}

impl AbilityDefinition for Reroll {
    fn name() -> String {
        "Reroll".into()
    }

    fn description() -> String {
        "Reroll {reroll_type} for {characteristic}".into()
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::Choice(ChoiceField {
                field_id: "characteristic".into(),
                display_name: "Characteristic".into(),
                choices: roll_characteristic_choices!(),
                default: None,
            }),
            Field::Choice(ChoiceField {
                field_id: "reroll_type".into(),
                display_name: "Reroll Type".into(),
                choices: vec![
                    Choice::new(&RerollType::Any.to_string(), "Any Roll"),
                    Choice::new(&RerollType::Failed.to_string(), "Failed Rolls"),
                    Choice::new(&RerollType::Ones.to_string(), "Rolls of 1"),
                ],
                default: None,
            }),
        ]
    }
}

impl AbilityDefinition for Bonus {
    fn name() -> String {
        "Bonus".into()
    }

    fn description() -> String {
        "Add {value} to {characteristic}".into()
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::Choice(ChoiceField {
                field_id: "characteristic".into(),
                display_name: "Characteristic".into(),
                choices: characteristic_choices!(),
                default: None,
            }),
            Field::DiceNotation(DiceNotationField {
                field_id: "value".into(),
                display_name: "Value".into(),
                default: None,
            }),
        ]
    }
}

impl AbilityDefinition for LeaderExtraAttacks {
    fn name() -> String {
        "Leader Extra Attacks".into()
    }

    fn description() -> String {
        "Add {value} to the leader of this unit ({models} leaders)".into()
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::DiceNotation(DiceNotationField {
                field_id: "value".into(),
                display_name: "Value".into(),
                default: None,
            }),
            Field::Number(NumberField {
                field_id: "models".into(),
                display_name: "Models".into(),
                min_value: Some(1),
                max_value: None,
                default: Some(1),
            }),
        ]
    }
}

impl AbilityDefinition for Exploding {
    fn name() -> String {
        "Exploding".into()
    }

    fn description() -> String {
        "{unmodified} rolls of {on}+ {characteristic} result in {extra} additional".into()
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::Choice(ChoiceField {
                field_id: "characteristic".into(),
                display_name: "Characteristic".into(),
                choices: roll_characteristic_choices!(),
                default: None,
            }),
            Field::Roll(RollField {
                field_id: "on".into(),
                display_name: "On".into(),
                min_value: 2,
                default: None,
            }),
            Field::Boolean(BooleanField {
                field_id: "unmodified".into(),
                display_name: "Unmodified".into(),
                default: Some(true),
            }),
            Field::DiceNotation(DiceNotationField {
                field_id: "extra".into(),
                display_name: "Extra".into(),
                default: None,
            }),
        ]
    }
}

impl AbilityDefinition for MortalWounds {
    fn name() -> String {
        "Mortal Wounds".into()
    }

    fn description() -> String {
        "{unmodified} rolls of {on}+ {characteritic} result in {mortals} mortal wounds \
        {in_addition}"
            .into()
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::Choice(ChoiceField {
                field_id: "characteristic".into(),
                display_name: "Characteristic".into(),
                choices: roll_characteristic_choices!(),
                default: None,
            }),
            Field::Roll(RollField {
                field_id: "on".into(),
                display_name: "On".into(),
                min_value: 2,
                default: None,
            }),
            Field::Boolean(BooleanField {
                field_id: "unmodified".into(),
                display_name: "Unmodified".into(),
                default: Some(true),
            }),
            Field::DiceNotation(DiceNotationField {
                field_id: "mortals".into(),
                display_name: "Mortal Wounds".into(),
                default: None,
            }),
            Field::Boolean(BooleanField {
                field_id: "in_addition".into(),
                display_name: "In Addition".into(),
                default: Some(false),
            }),
        ]
    }
}
