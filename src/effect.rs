use std::collections::HashMap;
use toml::value::Value;
use crate::installation::Installation;
use crate::pattern::Pattern;
use crate::light::Color;
use crate::fixture::{Element, ElementKind};
use crate::show_loader;

pub type GroupMap = HashMap<String, Vec<GroupElement>>;

#[derive(Debug)]
pub struct EffectPool {
    effects: Vec<Effect>,
    groups: GroupMap,
    installation: String,
    key_map: HashMap<String, String>,
    command_queue: Vec<Command>
}

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Toggle
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub key: String,
    pub action: Action,
}

#[derive(Debug)]
pub struct GroupElement {
    pub fixture: String,
    pub element: String,
}

#[derive(Debug)]
pub struct Effect {
    name: String,
    strength: f32,
    effect_elements: Vec<EffectElement>,
    effect_patterns: Vec<Pattern>,
}

#[derive(Debug)]
pub struct EffectElement {
    fixture: String,
    element: String,
    property: String,
    value: Value,
}

impl EffectElement {
    pub fn new(fixture: &str, element: &str, property: &str, value: &Value) -> Self {
        Self {
            fixture: fixture.to_owned(),
            element: element.to_owned(),
            property: property.to_owned(),
            value: value.clone()
        }
    }
}

impl EffectPool {
    pub fn new(effects: Vec<Effect>, groups: GroupMap, installation: String) -> Self {
        Self {
            effects,
            groups,
            installation,
            key_map: HashMap::new(),
            command_queue: vec![],
        }
    }

    pub fn set_key(&mut self, key: &str, effect_name: &str) {
        self.key_map.insert(key.to_owned(), effect_name.to_owned());
    }

    pub fn get_effect_by_key(&mut self, key: &str) -> Option<&mut Effect> {
        let name = self.key_map.get(key)?;
        self.effects.iter_mut().find(|effect| &effect.name == name)
    }

    pub fn new_from_config(config_file: &str) -> Self {
        show_loader::build_from_config(config_file)
    }

    pub fn apply_to(&mut self, installation: &mut Installation) {
        installation.zero();

        for effect in self.effects.iter_mut() {
            effect.apply_to(installation, &self.groups);
        }
    }

    pub fn installation(&self) -> &str {
        &self.installation
    }

    pub fn effects_mut(&mut self) -> &mut Vec<Effect> {
        &mut self.effects
    }

    pub fn add_commands(&mut self, mut commands: Vec<Command>) {
        self.command_queue.append(&mut commands);
    }

    pub fn run_commands(&mut self) {
        // todo This clone will become a problem when we want to mutate the
        // state of commands that last more than one "run" in the future.
        let queue = self.command_queue.clone();

        for command in queue.iter() {
            if let Some(effect) = self.get_effect_by_key(&command.key) {
                match command.action {
                    Action::Toggle => {
                        if effect.strength() > 0.0 {
                            effect.set_strength(0.0)
                        } else {
                            effect.set_strength(1.0)
                        }
                    },
                }
            }
        }

        self.command_queue.clear();
    }
}

impl Effect {
    pub fn new(name: &str, strength: f32, elements: Vec<EffectElement>,
               patterns: Vec<Pattern>) -> Self {
        Self {
            name: name.to_owned(),
            strength: strength,
            effect_elements: elements,
            effect_patterns: patterns,
        }
    }

    pub fn apply_to(&mut self, installation: &mut Installation, groups: &GroupMap) {
        let strength = self.strength;

        for effect_element in &self.effect_elements {
            let new_value = match effect_element.value {
                Value::Integer(value) => value as i32,
                _ => continue,
            };

            let (fixture, element) = (&effect_element.fixture, &effect_element.element);
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

        for pattern in self.effect_patterns.iter_mut() {
            let pattern_elements = groups.get(pattern.group()).unwrap();
            let new_values = pattern.update();

            for (effect_element, new_value) in pattern_elements.iter().zip(new_values.iter()) {
                let (fixture, element) = (&effect_element.fixture, &effect_element.element);
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

    pub fn strength(&self) -> f32 {
        self.strength
    }

    pub fn strength_mut(&mut self) -> &mut f32 {
        &mut self.strength
    }

    pub fn set_strength(&mut self, value: f32) {
        self.strength = value;
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
