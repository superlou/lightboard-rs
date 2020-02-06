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

pub fn build_from_config(config_file: &str) -> SceneManager {
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
                    scene_elements.push(SceneElement::new(
                        &fixture_name, &element_name, value
                    ));
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
                    groups.get(&group_name).unwrap().len(),
                    options
                );

                scene_patterns.push(pattern);
            }
        }

        Scene::new(&scene_config.name, 0.0, scene_elements, scene_patterns)
    }).collect();

    SceneManager::new(
        scenes, groups,
        config.installation.unwrap_or("installation.toml".to_owned()),
    )
}
