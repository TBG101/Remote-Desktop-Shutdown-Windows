#![windows_subsystem = "windows"]
use std::thread;

use crate::commands::hostname;
mod commands;
mod discovery;
mod network;
mod platform;
mod tray;
mod control;

fn main() {
    let hostname = match commands::hostname::get_hostname() {
        Ok(hostname) => hostname,
        Err(_) => panic!("Couldn't get hostname"),
    };

    if let Err(err) = discovery::advertise::advertise(&hostname) {
        // TODO: Log errors
    }

    thread::spawn(move || {
        let _ = network::server::packet_loop(&hostname);
    });

    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    tray::menu::build_menu(&mut app);

    _ = platform::startup::add_to_startup();

    _ = app.wait_for_message();
}
