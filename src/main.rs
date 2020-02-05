#[macro_use(lazy_static)]
extern crate lazy_static;

mod dmx_control;
mod gui;
mod imgui_wrapper;
mod installation;
mod fixture;
mod scene;
mod hitbox;
mod pattern;
mod light;

use std::{thread};
use std::sync::mpsc;
use clap::{Arg, App};
use installation::Installation;
use scene::SceneManager;

fn main() {
    let matches = App::new("Lightboard-rs")
                    .about("Rust DMX lighting controller")
                    .arg(Arg::with_name("scenes")
                            .help("Name of scenes file"))
                    .get_matches();

    println!("Started");

    let (send, recv) = mpsc::channel();

    let scenes_file = matches.value_of("scenes").unwrap_or("scenes.toml");
    let scene_manager = SceneManager::new(scenes_file);
    let installation = Installation::new(&scene_manager.installation);

    thread::spawn(move || { dmx_control::update(recv) });

    gui::run_gui(installation, scene_manager, send);
}
