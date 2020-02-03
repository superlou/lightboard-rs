use std::collections::HashMap;
use std::fs::read_to_string;
use serde::Deserialize;
use nalgebra::Point2;
use crate::fixture::{Fixture, Element, ElementKind};
use crate::light::Color;

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

impl Installation {
    pub fn new(config_file: &str) -> Installation {
        let config: InstallationConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        let fixtures: HashMap<_, _> = config.fixtures.into_iter().map(|(name, config)| {
            let (elements, num_channels) = load_elements(&config.kind, &config.mode);
            let fixture = Fixture::new(
                elements, Point2::new(config.pos.0, config.pos.1),
                config.channel as usize, num_channels
            );

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

    pub fn zero(&mut self) {
        for (_name, fixture) in self.fixtures.iter_mut() {
            for (_name, element) in fixture.elements_mut().iter_mut() {
                match element.kind() {
                    ElementKind::Rgbi(_) => {
                        element.set_kind(ElementKind::Rgbi(Color::black()));
                    },
                    ElementKind::Rgbiu{rgb: _, uv: _} => {
                        element.set_kind(ElementKind::Rgbiu{
                            rgb: Color::black(),
                            uv: 0.0,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn build_dmx_chain(&mut self) -> Vec<u8> {
        let mut chain = vec![];

        for (_name, fixture) in self.fixtures.iter_mut() {
            fixture.update_dmx();
            let fixture_dmx = fixture.dmx().to_vec();
            let channel = fixture.channel() - 1;

            let required_length = channel + fixture_dmx.len();

            if chain.len() < required_length {
                chain.resize(required_length, 0);
            }

            for (i, val) in fixture_dmx.iter().enumerate() {
                chain[channel + i] = *val;
            }
        }

        chain
    }
}
