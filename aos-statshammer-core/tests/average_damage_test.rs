use aos_statshammer_core::{abilities::*, processors::*, rollable::*, RollCharacteristic, Weapon};

macro_rules! processor_results {
    ($r1: expr, $r2: expr, $r3: expr, $r4: expr, $r5: expr, $r6: expr, $r7: expr) => {
        ProcessorResults::from([
            SaveResult::new(1, $r1),
            SaveResult::new(2, $r2),
            SaveResult::new(3, $r3),
            SaveResult::new(4, $r4),
            SaveResult::new(5, $r5),
            SaveResult::new(6, $r6),
            SaveResult::new(7, $r7),
        ])
    };
}

fn assert_average_damage_results_eq(output: ProcessorResults, expected: ProcessorResults) {
    assert_eq!(output.save_results.len(), expected.save_results.len());
    let round_precision = |value: f32, precision: i8| {
        if precision <= 0 {
            value.round()
        } else {
            let offset = 10.0 * (precision as f32);
            (value * offset).round() / offset
        }
    };

    for (index, expected_result) in expected.save_results.iter().enumerate() {
        assert_eq!(output.save_results[index].save, expected_result.save);
        let output_value = round_precision(output.save_results[index].value, 3);
        let expected_value = round_precision(expected_result.value, 3);
        assert_eq!(
            output_value, expected_value,
            "\n{:#?} != {:#?}",
            output, expected
        );
    }
}

#[test]
fn average_damage_gotrek_axe() {
    let weapon = Weapon::new(1, DiceNotation::from(6), 3, 3, 2, DiceNotation::from(3));
    let abilities = AbilityManager::new(vec![
        Ability::from(Reroll::new(RollCharacteristic::Hit)),
        Ability::from(Reroll::new(RollCharacteristic::Wound)),
        Ability::from(MortalWounds::new(
            RollCharacteristic::Hit,
            6,
            true,
            DiceNotation::try_from("d6").unwrap(),
            true,
        )),
    ]);
    let processor = AverageDamageProcessor::new(&weapon, &abilities);
    let output = processor.average_damage();
    assert_average_damage_results_eq(
        output,
        processor_results!(9.407, 11.778, 14.148, 16.519, 18.889, 18.889, 18.889),
    );
}

#[test]
fn average_hearthguard_berserkers_broadaxes() {
    let weapon = Weapon {
        models: 20,
        attacks: DiceNotation::from(2),
        to_hit: 3,
        to_wound: 3,
        rend: 1,
        damage: DiceNotation::from(2),
    };
    let abilities = AbilityManager::new(vec![Ability::from(LeaderExtraAttacks::new(
        DiceNotation::from(1),
        1,
    ))]);
    let processor = AverageDamageProcessor::new(&weapon, &abilities);
    let output = processor.average_damage();
    assert_average_damage_results_eq(
        output,
        processor_results!(6.074, 12.148, 18.222, 24.296, 30.370, 36.444, 36.444),
    );
}
