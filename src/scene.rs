use std::collections::HashMap;
use toml::value::Value;
use crate::installation::Installation;
use crate::pattern::Pattern;
use crate::light::Color;
use crate::fixture::{Element, ElementKind};
use crate::scene_manager_loader;

pub type GroupMap = HashMap<String, Vec<GroupElement>>;

#[derive(Debug)]
pub struct SceneManager {
    scenes: Vec<Scene>,
    groups: GroupMap,
    installation: String,
}

#[derive(Debug)]
pub struct GroupElement {
    pub fixture: String,
    pub element: String,
}

#[derive(Debug)]
pub struct Scene {
    name: String,
    strength: f32,
    scene_elements: Vec<SceneElement>,
    scene_patterns: Vec<Pattern>,
}

#[derive(Debug)]
pub struct SceneElement {
    fixture: String,
    element: String,
    value: Value,
}

impl SceneElement {
    pub fn new(fixture: &str, element: &str, value: Value) -> Self {
        Self {
            fixture: fixture.to_owned(),
            element: element.to_owned(),
            value: value
        }
    }
}

impl SceneManager {
    pub fn new(scenes: Vec<Scene>, groups: GroupMap, installation: String) -> Self {
        Self { scenes, groups, installation }
    }

    pub fn new_from_config(config_file: &str) -> Self {
        scene_manager_loader::build_from_config(config_file)
    }

    pub fn apply_to(&mut self, installation: &mut Installation) {
        installation.zero();

        for scene in self.scenes.iter_mut() {
            scene.apply_to(installation, &self.groups);
        }
    }

    pub fn installation(&self) -> &str {
        &self.installation
    }

    pub fn scenes_mut(&mut self) -> &mut Vec<Scene> {
        &mut self.scenes
    }
}

impl Scene {
    pub fn new(name: &str, strength: f32, elements: Vec<SceneElement>,
               patterns: Vec<Pattern>) -> Self {
        Self {
            name: name.to_owned(),
            strength: strength,
            scene_elements: elements,
            scene_patterns: patterns,
        }
    }

    pub fn apply_to(&mut self, installation: &mut Installation, groups: &GroupMap) {
        let strength = self.strength;

        for scene_element in &self.scene_elements {
            let new_value = match scene_element.value {
                Value::Integer(value) => value as i32,
                _ => continue,
            };

            let (fixture, element) = (&scene_element.fixture, &scene_element.element);
            let element = installation.find_element(fixture, element);
            let element = match element {
                Some(e) => e,
                None => continue,
            };

            let kind = mix_into_element_kind(element, new_value, strength);
            if let Some(kind) = kind {
                element.set_kind(kind);
            }
        }

        for pattern in self.scene_patterns.iter_mut() {
            let pattern_elements = groups.get(pattern.group()).unwrap();
            let new_values = pattern.update();

            for (scene_element, new_value) in pattern_elements.iter().zip(new_values.iter()) {
                let (fixture, element) = (&scene_element.fixture, &scene_element.element);
                let element = installation.find_element(fixture, element);
                let element = match element {
                    Some(e) => e,
                    None => continue,
                };

                let kind = mix_into_element_kind(element, *new_value, strength);
                if let Some(kind) = kind {
                    element.set_kind(kind);
                }
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn strength_mut(&mut self) -> &mut f32 {
        &mut self.strength
    }
}

fn mix_into_element_kind(element: &Element, new_value: i32, strength: f32) -> Option<ElementKind> {
    match element.kind() {
        ElementKind::Intensity(intensity) => {
            let effect_intensity = (new_value as i32 & 0xff) as f32 / 255.0 * strength;
            Some(ElementKind::Intensity(intensity + effect_intensity))
        }
        ElementKind::Rgbi(color) => {
            let mut effect_color: Color = (new_value as i32).into();
            effect_color.scale(strength);
            Some(ElementKind::Rgbi(color.clone() + effect_color))
        },
        ElementKind::Rgbiu{rgb: color, uv} => {
            let mut effect_color: Color = (new_value as i32).into();
            effect_color.scale(strength);
            Some(ElementKind::Rgbiu{rgb: color.clone() + effect_color, uv: *uv})
        },
        _=> None,
    }
}
