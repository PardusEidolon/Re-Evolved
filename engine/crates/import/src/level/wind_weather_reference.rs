//! The wind's palette entry is the game's: tools/engine_wind_weather_reference.py
//! runs wind.c's `scenario_get_wind` (and so `scenario_get_current` and
//! scenario.c's fog region lookups) over 400 made-up BSPs, recording the
//! weather palette entry each cluster's wind is blown from at points
//! above and below its fog plane, and [`wind_weather`] must choose the
//! same. The one departure is a fog plane's region, whose weather the
//! game takes below a plane of mist and the engine does not keep: those
//! cases are counted, and must be only those.

use super::{wind_weather, FogRegion};

const REFERENCE: &str = include_str!("../../../../../tools/traces/wind_weather_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// A BSP of the trace: its regions, whether each palette entry's fog is
/// water, and its fog planes' regions and heights.
#[derive(Default)]
struct World {
    regions: Vec<FogRegion>,
    palette: Vec<Option<bool>>,
    planes: Vec<(i16, f32)>,
}

impl World {
    /// The weather the game takes in place of the cluster's below a
    /// plane of mist at height `z`, which the engine does not keep.
    fn below_mist(&self, reference: i16, z: f32) -> Option<i16> {
        if reference == -1 || reference >= 0 {
            return None;
        }
        let (region, height) = *self
            .planes
            .get(usize::from(reference.cast_unsigned() & 0x7FFF))?;
        let region = self.regions.get(usize::try_from(region).ok()?)?;
        let water = *self.palette.get(usize::try_from(region.palette).ok()?)?;
        (water == Some(false) && region.weather != -1 && z - height < 0.0).then_some(region.weather)
    }
}

#[test]
fn the_wind_blows_from_the_games_palette_entry() -> Result<(), Failure> {
    let mut world = World::default();
    let (mut compared, mut departures) = (0, 0);
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.as_slice() {
            ["world", _] => world = World::default(),
            ["region", palette, weather] => world.regions.push(FogRegion {
                palette: palette.parse()?,
                weather: weather.parse()?,
            }),
            ["palette", fog] => world.palette.push(match *fog {
                "-1" => None,
                fog => Some(fog == "1"),
            }),
            ["plane", region, height] => world.planes.push((region.parse()?, height.parse()?)),
            ["wind", cluster, reference, z, game] => {
                let (cluster, reference, z, game): (i16, i16, f32, i16) = (
                    cluster.parse()?,
                    reference.parse()?,
                    z.parse()?,
                    game.parse()?,
                );
                let engine = wind_weather(cluster, reference, &world.regions, &world.palette);
                match world.below_mist(reference, z) {
                    Some(region) => {
                        assert_eq!(engine, cluster, "{line}");
                        assert_eq!(game, region, "{line}");
                        departures += 1;
                    }
                    None => assert_eq!(engine, game, "{line}"),
                }
                compared += 1;
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    assert_eq!(compared, 6940);
    assert!(
        departures > 0 && departures < compared / 10,
        "{departures} departures"
    );
    Ok(())
}
