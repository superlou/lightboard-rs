use std::collections::HashMap;
use std::fs::read_to_string;
use toml::value::Value;
use serde::Deserialize;
use crate::installation::Installation;
use crate::pattern::Pattern;
use crate::light::Color;
use crate::fixture::ElementKind;

pub type GroupMap = HashMap<String, Vec<GroupElement>>;

#[derive(Debug)]
pub struct SceneManager {
    pub scenes: Vec<Scene>,
    pub groups: GroupMap,
}

#[derive(Debug)]
pub struct GroupElement {
    fixture: String,
    element: String,
}

#[derive(Debug)]
pub struct Scene {
    pub name: String,
    pub strength: f32,
    pub scene_elements: Vec<SceneElement>,
    pub scene_patterns: Vec<Pattern>,
}

#[derive(Debug)]
pub struct SceneElement {
    fixture: String,
    element: String,
    value: Value,
}

#[derive(Deserialize, Debug)]
struct SceneManagerConfig {
    scenes: Vec<SceneConfig>,
    groups: HashMap<String, GroupConfig>,
}

#[derive(Deserialize, Debug)]
struct SceneConfig {
    name: String,
    fixtures: Option<HashMap<String, FixtureConfig>>,
    groups: Option<HashMap<String, GroupSceneConfig>>
}

#[derive(Deserialize, Debug)]
struct FixtureConfig {
    elements: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct ElementConfig {

}

#[derive(Deserialize, Debug)]
struct GroupConfig {
    elements: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct GroupSceneConfig {
    pattern: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct PatternConfig {
    name: String
}

impl SceneManager {
    pub fn new(config_file: &str) -> Self {
        let config: SceneManagerConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

        let groups: GroupMap = config.groups.into_iter().map(|(name, config)| {
            let group_elements = config.elements.iter().map(|s| {
                let parts: Vec<&str> = s.split(":").collect();
                GroupElement {
                    fixture: parts[0].to_owned(),
                    element: parts[1].to_owned(),
                }
            }).collect();

            (name, group_elements)
        }).collect();

        let scenes = config.scenes.into_iter().map(|scene_config| {
            let mut scene_elements = vec![];

            if let Some(fixtures) = scene_config.fixtures {
                for (fixture_name, fixture) in fixtures {
                    for (element_name, value) in fixture.elements {
                        scene_elements.push(SceneElement {
                            fixture: fixture_name.clone(),
                            element: element_name,
                            value: value,
                        });
                    }
                }
            }

            let mut scene_patterns = vec![];

            if let Some(scene_groups) = scene_config.groups {
                for (group_name, group) in scene_groups {
                    let mut options = group.pattern;
                    let script = match options.remove("script").unwrap() {
                        Value::String(s) => s,
                        _ => { println!("Expected string"); continue },
                    };

                    let pattern = Pattern::new(
                        &script,
                        &group_name,
                        groups.get(&group_name).unwrap().len()
                    );

                    scene_patterns.push(pattern);
                }
            }

            Scene {
                name: scene_config.name.to_owned(),
                strength: 0.0,
                scene_elements: scene_elements,
                scene_patterns: scene_patterns,
            }
        }).collect();

        Self {
            scenes: scenes,
            groups: groups,
        }
    }

    pub fn apply_to(&mut self, installation: &mut Installation) {
        installation.zero();

        for scene in self.scenes.iter_mut() {
            scene.apply_to(installation, &self.groups);
        }
    }
}

impl Scene {
    pub fn apply_to(&mut self, installation: &mut Installation, groups: &GroupMap) {
        let strength = self.strength;

        for scene_element in &self.scene_elements {
            let element = installation.fixtures_mut()
                            .get_mut(&scene_element.fixture).unwrap()
                            .elements.get_mut(&scene_element.element).unwrap();

            match scene_element.value {
                Value::Integer(value) => {
                    let kind = match element.kind() {
                        ElementKind::Rgbi(color) => {
                            let mut effect_color: Color = (value as i32).into();
                            effect_color.scale(strength);
                            Some(ElementKind::Rgbi(color.clone() + effect_color))
                        },
                        ElementKind::Rgbiu{rgb: color, uv} => {
                            let mut effect_color: Color = (value as i32).into();
                            effect_color.scale(strength);
                            Some(ElementKind::Rgbiu{rgb: color.clone() + effect_color, uv: *uv})
                        },
                        _ => { None },
                    };

                    if let Some(kind) = kind {
                        element.set_kind(kind);
                    }
                },
                _ => {}
            }
        }

        for pattern in self.scene_patterns.iter_mut() {
            let pattern_elements = groups.get(pattern.group()).unwrap();
            let new_values = pattern.update();

            for (scene_element, new_value) in pattern_elements.iter().zip(new_values.iter()) {
                let element = installation.fixtures_mut()
                                .get_mut(&scene_element.fixture).unwrap()
                                .elements.get_mut(&scene_element.element).unwrap();

                let kind = match element.kind() {
                    ElementKind::Rgbi(color) => {
                        let mut effect_color: Color = (*new_value as i32).into();
                        effect_color.scale(strength);
                        Some(ElementKind::Rgbi(color.clone() + effect_color))
                    },
                    ElementKind::Rgbiu{rgb: color, uv} => {
                        let mut effect_color: Color = (*new_value as i32).into();
                        effect_color.scale(strength);
                        Some(ElementKind::Rgbiu{rgb: color.clone() + effect_color, uv: *uv})
                    },
                    _=> { None },
                };

                if let Some(kind) = kind {
                    element.set_kind(kind);
                }
            }
        }
    }
}
