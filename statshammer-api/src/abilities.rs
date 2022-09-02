use aos_statshammer::abilities::{fields::Field, weapon, AbilityDefinition};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct AbilitySchema {
    id: String,
    name: String,
    description: String,
    fields: Vec<Field>,
}

#[derive(Serialize)]
pub struct AbilitiesResponse {
    weapon: Vec<AbilitySchema>,
}

macro_rules! ability_schema {
    ($id: expr, $ability: ty) => {
        AbilitySchema {
            id: $id.into(),
            name: <$ability>::name(),
            description: <$ability>::description(),
            fields: <$ability>::fields(),
        }
    };
}

pub async fn get_abilities() -> Json<AbilitiesResponse> {
    Json(AbilitiesResponse {
        weapon: vec![
            ability_schema!("Reroll", weapon::Reroll),
            ability_schema!("Bonus", weapon::Bonus),
            ability_schema!("LeaderExtraAttacks", weapon::LeaderExtraAttacks),
            ability_schema!("Exploding", weapon::Exploding),
            ability_schema!("MortalWounds", weapon::MortalWounds),
        ],
    })
}
