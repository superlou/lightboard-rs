#[macro_use(lazy_static)]
extern crate lazy_static;

mod dmx_control;
mod gui;
mod imgui_wrapper;
mod installation;
mod scene;
mod hitbox;

use std::{thread};
use std::sync::mpsc;

use installation::Installation;
use scene::SceneManager;

fn main() {
    println!("Started");

    let (send, recv) = mpsc::channel();

    let installation = Installation::new("installation.toml");
    let scene_manager = SceneManager::new("scenes.toml");

    thread::spawn(move || { dmx_control::update(recv) });

    gui::run_gui(installation, scene_manager, send);
}
