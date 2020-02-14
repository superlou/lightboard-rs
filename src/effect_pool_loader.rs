use std::collections::HashMap;
use toml::value::Value;
use serde::Deserialize;
use std::fs::read_to_string;
use crate::effect::{EffectPool, Effect, GroupMap, GroupElement, EffectElement};
use crate::pattern::Pattern;

#[derive(Deserialize, Debug)]
struct EffectPoolConfig {
    installation: Option<String>,
    effects: Vec<EffectConfig>,
    groups: HashMap<String, GroupConfig>,
}

#[derive(Deserialize, Debug)]
struct EffectConfig {
    name: String,
    elements: Option<Vec<HashMap<String, Value>>>,
    patterns: Option<Vec<HashMap<String, Value>>>,
}

#[derive(Deserialize, Debug)]
struct GroupConfig {
    elements: Vec<String>,
}

fn build_effect_element(config: &HashMap<String, Value>) -> Option<EffectElement> {
    let target = match config.get("target")? {
        Value::String(s) => s,
        _ => return None,
    };

    let value = config.get("color")?;
    let tokens: Vec<&str> = target.split(":").collect();
    let fixture = tokens[0];
    let element = tokens[1];
    let property = tokens[2];

    Some(EffectElement::new(fixture, element, property, value))
}

fn build_pattern(config: &mut HashMap<String, Value>, groups: &GroupMap) -> Option<Pattern> {
    let target = match config.remove("target")? {
        Value::String(s) => s,
        _ => return None,
    };

    let tokens: Vec<&str> = target.split(":").collect();
    let target = tokens[0];
    let property = tokens[1];

    let group_name = &target[1..];  // todo Don't assume ASCII

    let script = match config.remove("script")? {
        Value::String(s) => s,
        _ => return None,
    };

    let options = config.clone();
    let num_group_elements = groups.get(group_name)?.len();

    Some(Pattern::new(&script, group_name, property, num_group_elements, options))
}

fn build_group_elements(config: &GroupConfig) -> Vec<GroupElement> {
    config.elements.iter().map(|s| {
        let parts: Vec<&str> = s.split(":").collect();
        GroupElement {
            fixture: parts[0].to_owned(),
            element: parts[1].to_owned(),
        }
    }).collect()
}

pub fn build_from_config(config_file: &str) -> EffectPool {
    let config: EffectPoolConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

    let groups: GroupMap = config.groups.into_iter().map(|(name, config)| {
        (name, build_group_elements(&config))
    }).collect();

    let effects = config.effects.into_iter().map(|effect_config| {
        let elements = effect_config.elements.unwrap_or(vec![]).iter()
            .filter_map(|c| build_effect_element(c))
            .collect();

        let patterns = effect_config.patterns.unwrap_or(vec![]).iter_mut()
            .filter_map(|mut c| build_pattern(&mut c, &groups))
            .collect();

        Effect::new(&effect_config.name, 0.0, elements, patterns)
    }).collect();

    EffectPool::new(
        effects, groups,
        config.installation.unwrap_or("installation.toml".to_owned()),
    )
}
