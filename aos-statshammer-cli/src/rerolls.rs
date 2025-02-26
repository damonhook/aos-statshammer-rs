use aos_statshammer_core as core;
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RerollType {
    Ones,
    Failed,
    Any,
}
impl From<RerollType> for core::RerollType {
    fn from(value: RerollType) -> Self {
        match value {
            RerollType::Ones => core::RerollType::Ones,
            RerollType::Failed => core::RerollType::Failed,
            RerollType::Any => core::RerollType::Any,
        }
    }
}
