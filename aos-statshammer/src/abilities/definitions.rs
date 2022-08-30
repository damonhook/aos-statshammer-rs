use super::fields::*;
use super::{Bonus, Exploding, LeaderExtraAttacks, MortalWounds, Reroll, RerollFailed, RerollOnes};
use aos_statshammer_core::{ValueCharacteristic as VChar, RollCharacteristic as RChar};

pub trait AbilityDefinition {
    /// A display name for the Ability
    fn name() -> String;
    /// A list of [Fields](Field) defining how the specific ability can configured
    fn fields() -> Vec<Field>;
}

macro_rules! characteristic_choices {
    () => {
        vec![
            Choice::new(&VChar::Attacks.to_string(), "Attacks"),
            Choice::new(&RChar::Hit.to_string(), "To Hit"),
            Choice::new(&RChar::Wound.to_string(), "To Wound"),
            Choice::new(&VChar::Rend.to_string(), "Rend"),
            Choice::new(&VChar::Damage.to_string(), "Damage"),
        ]
    };
}

macro_rules! roll_characteristic_choices {
    () => {
        vec![
            Choice::new(&RChar::Hit.to_string(), "To Hit"),
            Choice::new(&RChar::Wound.to_string(), "To Wound"),
        ]
    };
}

impl AbilityDefinition for Reroll {
    fn name() -> String {
        "Reroll".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::Choice(ChoiceField {
            field_id: "characteristic".into(),
            display_name: "Characteristic".into(),
            choices: roll_characteristic_choices!(),
            default: None,
        })]
    }
}

impl AbilityDefinition for RerollFailed {
    fn name() -> String {
        "Reroll Failed".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::Choice(ChoiceField {
            field_id: "characteristic".into(),
            display_name: "Characteristic".into(),
            choices: roll_characteristic_choices!(),
            default: None,
        })]
    }
}

impl AbilityDefinition for RerollOnes {
    fn name() -> String {
        "Reroll Ones".into()
    }

    fn fields() -> Vec<Field> {
        vec![Field::Choice(ChoiceField {
            field_id: "characteristic".into(),
            display_name: "Characteristic".into(),
            choices: roll_characteristic_choices!(),
            default: None,
        })]
    }
}

impl AbilityDefinition for Bonus {
    fn name() -> String {
        "Bonus".into()
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

    fn fields() -> Vec<Field> {
        vec![
            Field::DiceNotation(DiceNotationField {
                field_id: "value".into(),
                display_name: "Value".into(),
                default: None,
            }),
            Field::Number(NumberField {
                field_id: "num_models".into(),
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
