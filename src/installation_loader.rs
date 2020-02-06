use std::collections::HashMap;
use serde::Deserialize;
use std::fs::read_to_string;
use nalgebra::Point2;
use crate::installation::Installation;
use crate::fixture::{Fixture, Element, ElementKind};
use crate::light::Color;

#[derive(Deserialize, Debug)]
struct InstallationConfig {
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
    num_channels: usize,
    elements: HashMap<String, ElementConfig>
}

#[derive(Deserialize, Debug, Clone)]
struct ElementConfig {
    kind: String,
    i: Option<u8>,
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
    uv: Option<u8>,
}

impl From<ElementConfig> for Element {
    fn from(config: ElementConfig) -> Self {
        let kind = match config.kind.as_str() {
            "rgbiu" => ElementKind::Rgbiu{rgb: Color::black(), uv: 1.0},
            "rgbi" => ElementKind::Rgbi(Color::black()),
            "u" => ElementKind::Uv(0.0),
            "smoke" => ElementKind::Smoke,
            "gobo" => ElementKind::Gobo,
            "actuator" => ElementKind::Actuator,
            _ => ElementKind::Unknown,
        };

        let mut element = Element::new(kind);

        if let Some(channel) = config.i {
            element.add_channel("i", channel);
        }

        if let Some(channel) = config.r {
            element.add_channel("r", channel);
        }

        if let Some(channel) = config.g {
            element.add_channel("g", channel);
        }

        if let Some(channel) = config.b {
            element.add_channel("b", channel);
        }

        if let Some(channel) = config.uv {
            element.add_channel("uv", channel);
        }

        element
    }
}

fn load_elements(kind: &str, mode: &str) -> (HashMap<String, Element>, usize) {
    let fixture_path = "fixtures/".to_owned() + kind + ".toml";
    let mut config: FixtureDefConfig = toml::from_str(&read_to_string(fixture_path).unwrap()).unwrap();

    config.modes.retain(|mode_config| mode_config.name == mode);

    if config.modes.len() >= 1 {
        let elements = config.modes[0].elements.clone();
        (elements.into_iter().map(|(name, config)| {
            (name, config.into())
        }).collect(), config.modes[0].num_channels)
    } else {
        (HashMap::new(), 0)
    }
}

pub fn build_from_config(config_file: &str) -> Installation {
    let config_text = read_to_string(config_file)
                        .expect(&format!("Failed to find {}", config_file));
    let config: InstallationConfig = toml::from_str(&config_text)
                        .expect(&format!("Failed to parse {}", config_file));

    let fixtures: HashMap<_, _> = config.fixtures.into_iter().map(|(name, config)| {
        let (elements, num_channels) = load_elements(&config.kind, &config.mode);
        let fixture = Fixture::new(
            elements, Point2::new(config.pos.0, config.pos.1),
            config.channel as usize, num_channels
        );

        (name, fixture)
    }).collect();

    Installation::new(fixtures)
}
