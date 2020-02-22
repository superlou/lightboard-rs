#[macro_use(lazy_static)]
extern crate lazy_static;

mod dmx_control;
mod gui;
mod imgui_wrapper;
mod installation;
mod fixture;
mod effect;
mod hitbox;
mod pattern;
mod light;
mod installation_loader;
mod show_loader;

use std::{thread};
use std::sync::mpsc;
use clap::{Arg, App};
use installation::Installation;
use effect::EffectPool;

fn main() {
    let matches = App::new("Lightboard-rs")
                    .about("Rust DMX lighting controller")
                    .arg(Arg::with_name("show")
                            .help("Name of show file"))
                    .get_matches();

    println!("Started");

    let (send, recv) = mpsc::channel();

    let show_file = matches.value_of("show").unwrap_or("show.toml");
    let effect_pool = EffectPool::new_from_config(show_file);
    let installation = Installation::new_from_config(&effect_pool.installation());

    thread::spawn(move || { dmx_control::update(recv) });

    gui::run_gui(installation, effect_pool, send);
}
