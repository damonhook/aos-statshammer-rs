#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RerollType {
    Ones,
    Failed,
    Any,
}

impl fmt::Display for RerollType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ones => write!(f, "Ones"),
            Self::Failed => write!(f, "Failed"),
            Self::Any => write!(f, "Any"),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! create_abilities_enum {
    {
        enum_name=$enum_name: ident,
        abilities=[$($ability: ident),+],
        used_for_doclink=$used_for: tt
    } => {
        #[doc = concat!(
            "An enum represeting all of the possible abilities for a ",
            stringify!($used_for),
            "."
        )]
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(
            feature = "serde",
            derive(Serialize, Deserialize),
            serde(tag = "type", content = "value")
        )]
        pub enum $enum_name {
            $($ability($ability),)*
        }

        $(impl From<$ability> for $enum_name {
            fn from(a: $ability) -> Self {
                Self::$ability(a)
            }
        })*
    };
}

pub(crate) use create_abilities_enum;
