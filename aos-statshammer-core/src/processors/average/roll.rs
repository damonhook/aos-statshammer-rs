use crate::abilities::weapon::RerollType;

use super::RollTarget;

pub fn probability(target: f32) -> f32 {
    if target > 7.0 {
        0.0
    } else {
        ((7.0 - target) / 6.0).clamp(0.0, 1.0)
    }
}

pub fn inverse_probability(target: f32) -> f32 {
    1.0 - probability(target)
}

pub fn reroll_probability(reroll_type: RerollType, base: f32, target: RollTarget<f32>) -> f32 {
    match reroll_type {
        RerollType::Any => base * inverse_probability(target.modified()),
        RerollType::Failed => base * inverse_probability(target.unmodified()),
        RerollType::Ones => base / 6.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;
    use test_case::test_case;

    static PRECISION: f32 = 0.000_5; // Approximately 3 decimal places

    #[test_case(1.0, 1.0    ; "0+")]
    #[test_case(1.0, 1.0    ; "1+")]
    #[test_case(2.0, 0.833  ; "2+")]
    #[test_case(3.0, 0.667  ; "3+")]
    #[test_case(4.0, 0.5    ; "4+")]
    #[test_case(5.0, 0.333  ; "5+")]
    #[test_case(6.0, 0.167  ; "6+")]
    #[test_case(7.0, 0.0    ; "7+")]
    fn roll_probability_for_target(target: f32, expected: f32) {
        let output = probability(target);
        assert_float_eq!(output, expected, abs <= 0.0005);
    }

    #[test_case(1.0, 0.0    ; "0-")]
    #[test_case(1.0, 0.0    ; "1-")]
    #[test_case(2.0, 0.167  ; "2-")]
    #[test_case(3.0, 0.333  ; "3-")]
    #[test_case(4.0, 0.5    ; "4-")]
    #[test_case(5.0, 0.667  ; "5-")]
    #[test_case(6.0, 0.833  ; "6-")]
    #[test_case(7.0, 1.0    ; "7-")]
    fn inverse_probability_for_target(target: f32, expected: f32) {
        let output = inverse_probability(target);
        assert_float_eq!(output, expected, abs <= PRECISION);
    }
}
