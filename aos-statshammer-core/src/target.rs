use crate::RerollType;
use crate::characteristics::*;
use crate::dice::D6;
use derive_builder::Builder;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Save {
    pub value: u8,
    pub bonus: i16,
    pub reroll: Option<RerollType>,
}
impl_characteristic!(Save, value, bonus, -);
impl_reroll!(Save, reroll);

#[derive(Debug, Builder)]
pub struct Target {
    #[builder(setter(into), default)]
    pub save: Save,
    #[builder(default)]
    pub ethereal: bool,
}

impl Target {
    pub(crate) fn average_saved(&self, rend: u8) -> f64 {
        let target = self.save - rend as i16;
        if self.ethereal {
            D6.probability(target.unmodified())
        } else {
            D6.probability(target.modified())
        }
    }

    pub(crate) fn average_unsaved(&self, rend: u8) -> f64 {
        1.0 - self.average_saved(rend)
    }
}

impl<T> From<T> for Target
where
    T: Into<Save>,
{
    fn from(save: T) -> Self {
        Self {
            save: save.into(),
            ethereal: false,
        }
    }
}

impl TargetBuilder {
    pub fn bonus(&mut self, bonus: i16) -> &mut Self {
        self.save = Some(Save {
            bonus,
            ..self.save.unwrap_or_default()
        });
        self
    }

    pub fn reroll(&mut self, reroll: RerollType) -> &mut Self {
        self.save = Some(Save {
            reroll: Some(reroll),
            ..self.save.unwrap_or_default()
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const MAX_RELATIVE: f64 = 0.001;

    #[test]
    fn test_average_saved_basic_no_rend() {
        let target = Target {
            save: 4.into(),
            ethereal: false,
        };
        assert_relative_eq!(target.average_saved(0), 0.5);
    }

    #[test]
    fn test_average_unsaved_inverse_of_saved() {
        let target = Target {
            save: 4.into(),
            ethereal: false,
        };
        assert_relative_eq!(target.average_unsaved(0), 0.5);
    }

    #[test]
    fn test_average_saved_basic_with_rend() {
        let target = Target {
            save: 4.into(),
            ethereal: false,
        };
        assert_relative_eq!(target.average_saved(1), 0.333, max_relative = MAX_RELATIVE);
    }

    #[test]
    fn test_average_saved_with_bonus_and_rend() {
        let target = Target {
            save: Save {
                value: 4,
                bonus: 1,
                reroll: None,
            },
            ethereal: false,
        };
        assert_relative_eq!(target.average_saved(1), 0.5);
    }

    #[test]
    fn test_average_saved_ethereal_ignores_bonus() {
        let target = Target {
            save: Save {
                value: 4,
                bonus: 1,
                reroll: None,
            },
            ethereal: true,
        };
        assert_relative_eq!(target.average_saved(1), 0.5);
    }

    #[test]
    fn test_average_saved_ethereal_ignores_rend() {
        let target = Target {
            save: 4.into(),
            ethereal: true,
        };
        assert_relative_eq!(target.average_saved(1), 0.5);
    }

    #[test]
    fn test_average_saved_reroll_ones() {
        let target = Target {
            save: 4.into(),
            ethereal: true,
        };
        assert_relative_eq!(target.average_saved(1), 0.5);
    }
}
