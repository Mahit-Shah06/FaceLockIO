mod device_manager;
mod core_controller;
mod vision;
mod security_gate;
mod input_controller;

use std::process;

fn main() {
    println!("FSRC Backend starting");
    if unsafe { libc::getuid() } != 0 {
        eprintln!("Error: This program must be run as root (sudo) to control input devices.");
        process::exit(1);
    }

    core_controller::start();
}
