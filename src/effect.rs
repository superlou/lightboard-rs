use std::collections::HashMap;
use toml::value::Value;
use crate::installation::Installation;
use crate::pattern::Pattern;
use crate::light::Color;
use crate::fixture::{Element, ElementKind};
use crate::effect_pool_loader;

pub type GroupMap = HashMap<String, Vec<GroupElement>>;

#[derive(Debug)]
pub struct EffectPool {
    effects: Vec<Effect>,
    groups: GroupMap,
    installation: String,
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
        Self { effects, groups, installation }
    }

    pub fn new_from_config(config_file: &str) -> Self {
        effect_pool_loader::build_from_config(config_file)
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
