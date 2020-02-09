use std::collections::HashMap;
use toml::value::Value;
use serde::Deserialize;
use std::fs::read_to_string;
use crate::scene::{SceneManager, Scene, GroupMap, GroupElement, SceneElement};
use crate::pattern::Pattern;

#[derive(Deserialize, Debug)]
struct SceneManagerConfig {
    installation: Option<String>,
    scenes: Vec<SceneConfig>,
    groups: HashMap<String, GroupConfig>,
}

#[derive(Deserialize, Debug)]
struct SceneConfig {
    name: String,
    elements: Option<Vec<HashMap<String, Value>>>,
    patterns: Option<Vec<HashMap<String, Value>>>,
    // fixtures: Option<HashMap<String, FixtureConfig>>,
    // groups: Option<HashMap<String, GroupSceneConfig>>
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

fn build_scene_element(config: &HashMap<String, Value>) -> Option<SceneElement> {
    let target = match config.get("target")? {
        Value::String(s) => s,
        _ => return None,
    };

    let value = config.get("color")?;
    let tokens: Vec<&str> = target.split(":").collect();
    let fixture = tokens[0];
    let element = tokens[1];

    Some(SceneElement::new(fixture, element, value.clone()))
}

fn build_pattern(config: &mut HashMap<String, Value>, groups: &GroupMap) -> Option<Pattern> {
    let target = match config.remove("target")? {
        Value::String(s) => s,
        _ => return None,
    };

    let group_name = &target[1..];  // todo Don't assume ASCII

    let script = match config.remove("script")? {
        Value::String(s) => s,
        _ => return None,
    };

    let options = config.clone();
    let num_group_elements = groups.get(group_name)?.len();

    Some(Pattern::new(&script, group_name, num_group_elements, options))
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

pub fn build_from_config(config_file: &str) -> SceneManager {
    let config: SceneManagerConfig = toml::from_str(&read_to_string(config_file).unwrap()).unwrap();

    let groups: GroupMap = config.groups.into_iter().map(|(name, config)| {
        (name, build_group_elements(&config))
    }).collect();

    let scenes = config.scenes.into_iter().map(|scene_config| {
        let elements = scene_config.elements.unwrap_or(vec![]).iter()
            .filter_map(|c| build_scene_element(c))
            .collect();

        let patterns = scene_config.patterns.unwrap_or(vec![]).iter_mut()
            .filter_map(|mut c| build_pattern(&mut c, &groups))
            .collect();

        Scene::new(&scene_config.name, 0.0, elements, patterns)
    }).collect();

    SceneManager::new(
        scenes, groups,
        config.installation.unwrap_or("installation.toml".to_owned()),
    )
}
