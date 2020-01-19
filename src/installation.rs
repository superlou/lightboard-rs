use std::collections::HashMap;
use std::fs::read_to_string;
use serde::Deserialize;
use nalgebra::Point2;

enum ElementType {
    Rgbiu,
    Rgbi,
    Uv,
    Smoke,
    Actuator,
    Gobo,
}

pub struct Element {
    kind: ElementType,
}

pub struct Fixture {
    elements: HashMap<String, Element>,
    pub pos: Point2<f32>,
}

pub struct Installation {
    size: (f32, f32),
    fixtures: HashMap<String, Fixture>
}

#[derive(Deserialize, Debug)]
struct InstallationConfig {
    size: (f32, f32),
    fixtures: HashMap<String, FixtureConfig>,
}

#[derive(Deserialize, Debug)]
struct FixtureConfig {
    kind: String,
    channel: u8,
    mode: u8,
    pos: (f32, f32),
}

impl Installation {
    pub fn new(config_file: &str) -> Installation {
        let config: InstallationConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();
        dbg!(&config);

        let fixtures: HashMap<_, _> = config.fixtures.into_iter().map(|(name, config)| {
            let fixture = Fixture {
                elements: HashMap::new(),
                pos: Point2::new(config.pos.0, config.pos.1),
            };
            (name, fixture)
        }).collect();

        Installation {
            size: config.size,
            fixtures: fixtures,
        }
    }

    pub fn fixtures(&self) -> &HashMap<String, Fixture> {
        &self.fixtures
    }

    pub fn size(&self) -> &(f32, f32) {
        &self.size
    }
}
