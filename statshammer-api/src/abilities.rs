use aos_statshammer::abilities::{
    fields::Field, AbilityDefinition, Bonus, Exploding, LeaderExtraAttacks, MortalWounds, Reroll,
    RerollFailed, RerollOnes,
};
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
    abilities: Vec<AbilitySchema>,
}

macro_rules! ability_schema {
    ($id: expr, $ability: ident) => {
        AbilitySchema {
            id: $id.into(),
            name: $ability::name(),
            description: $ability::description(),
            fields: $ability::fields(),
        }
    };
}

pub async fn get_abilities() -> Json<AbilitiesResponse> {
    Json(AbilitiesResponse {
        abilities: vec![
            ability_schema!("reroll", Reroll),
            ability_schema!("reroll_failed", RerollFailed),
            ability_schema!("reroll_ones", RerollOnes),
            ability_schema!("bonus", Bonus),
            ability_schema!("leader_extra_attacks", LeaderExtraAttacks),
            ability_schema!("exploding", Exploding),
            ability_schema!("mortal_wounds", MortalWounds),
        ],
    })
}
