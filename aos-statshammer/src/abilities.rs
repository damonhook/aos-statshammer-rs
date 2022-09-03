pub use aos_statshammer_core::abilities::RerollType;

pub mod fields;
pub trait AbilityDefinition {
    /// A display name for the Ability
    fn name() -> String;
    /// A description for the Ability with placeholders for where field values would be substituded
    fn description() -> String;
    /// A list of [Fields](Field) defining how the specific ability can configured
    fn fields() -> Vec<fields::Field>;
}

pub mod opponent;
pub mod weapon;
