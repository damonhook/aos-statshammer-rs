use crate::rerolls::RerollType;
use aos_statshammer_core as core;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TargetConfig {
    characteristics: Characteristics,
    #[serde(default)]
    abilities: Abilities,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Characteristics {
    save: u8,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Abilities {
    bonus: Option<i16>,
    ethereal: bool,
    reroll: Option<RerollType>,
}

impl TryInto<core::target::Target> for TargetConfig {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<core::target::Target, Self::Error> {
        let mut builder = core::target::TargetBuilder::default();
        builder
            .save(self.characteristics.save)
            .ethereal(self.abilities.ethereal);
        if let Some(bonus) = self.abilities.bonus {
            builder.bonus(bonus);
        }
        if let Some(reroll) = self.abilities.reroll {
            builder.reroll(reroll.into());
        }
        builder.build().map_err(Into::into)
    }
}
