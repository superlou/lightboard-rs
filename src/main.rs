#[macro_use(lazy_static)]
extern crate lazy_static;

mod dmx_control;
mod gui;
mod imgui_wrapper;
mod installation;

use std::{thread};
use std::sync::mpsc;

use installation::Installation;

fn main() {
    println!("Started");

    let (send, recv) = mpsc::channel();

    let installation = Installation::new("installation.toml");

    thread::spawn(move || { dmx_control::update(recv) });

    gui::run_gui(installation, send);

    // loop {
    //     let mut s = String::new();
    //     stdin().read_line(&mut s);
    //     let value = s.trim().parse().unwrap();
    //     match send.send(value) {
    //         Ok(_) => {},
    //         Err(_) => println!("Unable to send"),
    //     };
    // }
}
