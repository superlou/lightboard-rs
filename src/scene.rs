use std::collections::HashMap;
use std::fs::read_to_string;
use toml::value::Value;
use serde::Deserialize;
use crate::installation::Installation;

#[derive(Debug)]
pub struct SceneManager {
    pub scenes: Vec<Scene>,
}

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub strength: f32,
    pub scene_elements: Vec<SceneElement>,
}

#[derive(Debug)]
pub struct SceneElement {
    fixture_name: String,
    element_name: String,
    value: Value,
}

#[derive(Deserialize, Debug)]
struct SceneManagerConfig {
    scenes: Vec<SceneConfig>,
}

#[derive(Deserialize, Debug)]
struct SceneConfig {
    name: String,
    fixtures: HashMap<String, FixtureConfig>,
}

#[derive(Deserialize, Debug)]
struct FixtureConfig {
    elements: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct ElementConfig {

}

impl SceneManager {
    pub fn new(config_file: &str) -> Self {
        let config: SceneManagerConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        dbg!(&config);

        let scenes = config.scenes.into_iter().map(|scene_config| {
            let mut scene_elements = vec![];

            for (fixture_name, fixture) in scene_config.fixtures {
                for (element_name, value) in fixture.elements {
                    scene_elements.push(SceneElement {
                        fixture_name: fixture_name.clone(),
                        element_name: element_name,
                        value: value,
                    });
                }
            }

            Scene {
                name: scene_config.name.to_owned(),
                strength: 0.0,
                scene_elements: scene_elements
            }
        }).collect();

        dbg!(&scenes);

        Self {
            scenes: scenes,
        }
    }

    pub fn apply_to(&self, installation: &mut Installation) {
        installation.zero();

        for scene in self.scenes.iter() {
            scene.apply_to(installation);
        }
    }
}

impl Scene {
    pub fn apply_to(&self, installation: &mut Installation) {
        let strength = self.strength;

        for scene_element in &self.scene_elements {
            let element = installation.fixtures_mut()
                            .get_mut(&scene_element.fixture_name).unwrap()
                            .elements.get_mut(&scene_element.element_name).unwrap();

            match scene_element.value {
                Value::Integer(value) => {
                    let r = ((value >> 16) & 0xff) as f32 / 255.0;
                    let g = ((value >> 8) & 0xff) as f32 / 255.0;
                    let b = (value & 0xff) as f32 / 255.0;

                    element.set_intensity(1.0);

                    let (r0, g0, b0) = element.color();

                    let mut r = r * strength + r0;
                    let mut g = g * strength + g0;
                    let mut b = b * strength + b0;

                    if r > 1.0 { r = 1.0 }
                    if g > 1.0 { g = 1.0 }
                    if b > 1.0 { b = 1.0 }

                    element.set_color(r, g, b);
                },
                _ => {}
            }
        }
    }
}
