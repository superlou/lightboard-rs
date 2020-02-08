use std::collections::HashMap;
use crate::fixture::{Fixture, ElementKind};
use crate::light::Color;
use crate::installation_loader;

pub struct Installation {
    fixtures: HashMap<String, Fixture>
}

impl Installation {
    pub fn new(fixtures: HashMap<String, Fixture>) -> Self {
        Self { fixtures }
    }

    pub fn new_from_config(config_file: &str) -> Installation {
        installation_loader::build_from_config(config_file)
    }

    pub fn fixtures(&self) -> &HashMap<String, Fixture> {
        &self.fixtures
    }

    pub fn fixtures_mut(&mut self) -> &mut HashMap<String, Fixture> {
        &mut self.fixtures
    }

    pub fn zero(&mut self) {
        for (_name, fixture) in self.fixtures.iter_mut() {
            for (_name, element) in fixture.elements_mut().iter_mut() {
                match element.kind() {
                    ElementKind::Intensity(_) => element.set_kind(ElementKind::Intensity(0.0)),
                    ElementKind::Rgbi(_) => element.set_kind(ElementKind::Rgbi(Color::black())),
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
