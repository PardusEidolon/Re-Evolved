//! The trigger volumes are the game's: tools/engine_scenario_reference.py
//! runs scenario.c's `scenario_trigger_volume_test_point` over forty
//! volumes, half along the world's axes and half turned by their own, at
//! points about, inside and on them and at indices the scenario does not
//! have, and the engine's volumes are asked the same and compared.

use opence_content::TriggerVolume;

const REFERENCE: &str = include_str!("../../../../tools/traces/scenario_reference.txt");

type Failure = Box<dyn std::error::Error>;

fn reals(fields: &[&str]) -> Result<Vec<f32>, Failure> {
    Ok(fields
        .iter()
        .map(|field| field.parse::<f32>())
        .collect::<Result<_, _>>()?)
}

fn triple(values: &[f32]) -> Result<[f32; 3], Failure> {
    match values {
        [x, y, z] => Ok([*x, *y, *z]),
        _ => Err("not three reals".into()),
    }
}

/// Each line of a kind, split after its name.
fn lines(kind: &str) -> impl Iterator<Item = Vec<&str>> + '_ {
    REFERENCE
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(move |fields| fields.first() == Some(&kind))
        .map(|fields| fields[1..].to_vec())
}

/// The trace's volumes, as the engine imports them.
fn volumes() -> Result<Vec<TriggerVolume>, Failure> {
    lines("volume")
        .map(|fields| {
            let values = reals(fields.get(2..14).ok_or("a short line")?)?;
            Ok(TriggerVolume {
                name: String::new(),
                oriented: fields.get(1) == Some(&"1"),
                position: triple(&values[0..3])?,
                forward: triple(&values[3..6])?,
                up: triple(&values[6..9])?,
                extents: triple(&values[9..12])?,
            })
        })
        .collect()
}

#[test]
fn a_point_is_inside_a_trigger_volume_as_the_games() -> Result<(), Failure> {
    let volumes = volumes()?;
    assert_eq!(volumes.len(), 40);
    let (mut compared, mut inside) = (0, 0);
    for fields in lines("test") {
        let index: i32 = fields.first().ok_or("a short line")?.parse()?;
        let point = triple(&reals(fields.get(1..4).ok_or("a short line")?)?)?;
        let expected = fields.get(4) == Some(&"1");
        let actual = usize::try_from(index)
            .ok()
            .and_then(|index| volumes.get(index))
            .is_some_and(|volume| volume.contains(point));
        assert_eq!(actual, expected, "volume {index} point {point:?}");
        compared += 1;
        inside += usize::from(expected);
    }
    assert_eq!(compared, 44 * 60);
    assert!(inside > 500, "{inside} inside");
    Ok(())
}
