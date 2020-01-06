use std::collections::HashMap;
use std::fs::read_to_string;
use serde::Deserialize;

pub struct Fixture {

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

        Installation {
            size: config.size,
            fixtures: HashMap::new()
        }
    }
}
