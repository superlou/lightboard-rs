use std::collections::HashMap;
use std::fs::read_to_string;
use serde::Deserialize;
use nalgebra::Point2;

#[derive(Debug)]
enum ElementType {
    Rgbiu,
    Rgbi,
    Uv,
    Smoke,
    Actuator,
    Gobo,
    Unknown,
}

#[derive(Debug)]
pub struct Element {
    kind: ElementType,
    color: (f32, f32, f32),
    intensity: f32,
}

#[derive(Debug)]
pub struct Fixture {
    pub elements: HashMap<String, Element>,
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
    mode: String,
    pos: (f32, f32),
}

#[derive(Deserialize, Debug)]
struct FixtureDefConfig {
    modes: Vec<ModeConfig>
}

#[derive(Deserialize, Debug)]
struct ModeConfig {
    name: String,
    elements: HashMap<String, ElementConfig>
}

#[derive(Deserialize, Debug, Clone)]
struct ElementConfig {
    kind: String
}

impl From<ElementConfig> for Element {
    fn from(config: ElementConfig) -> Self {
        let kind = match config.kind.as_str() {
            "rgbiu" => ElementType::Rgbiu,
            "rgbi" => ElementType::Rgbi,
            "u" => ElementType::Uv,
            _ => ElementType::Unknown,
        };

        Element {
            kind: kind,
            color: (0.0, 0.0, 0.0),
            intensity: 0.0,
        }
    }
}

fn load_elements(kind: &str, mode: &str) -> HashMap<String, Element> {
    let fixture_path = "fixtures/".to_owned() + kind + ".toml";
    let mut config: FixtureDefConfig = toml::from_str(&read_to_string(fixture_path).unwrap()).unwrap();

    config.modes.retain(|mode_config| mode_config.name == mode);

    if config.modes.len() >= 1 {
        let elements = config.modes[0].elements.clone();
        elements.into_iter().map(|(name, config)| {
            (name, config.into())
        }).collect()
    } else {
        HashMap::new()
    }
}

impl Installation {
    pub fn new(config_file: &str) -> Installation {
        let config: InstallationConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        let fixtures: HashMap<_, _> = config.fixtures.into_iter().map(|(name, config)| {
            let fixture = Fixture {
                elements: load_elements(&config.kind, &config.mode),
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

    pub fn fixtures_mut(&mut self) -> &mut HashMap<String, Fixture> {
        &mut self.fixtures
    }

    pub fn size(&self) -> &(f32, f32) {
        &self.size
    }
}

impl Element {
    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = (r, g, b);
    }

    pub fn set_intensity(&mut self, i: f32) {
        self.intensity = i;
    }
}
