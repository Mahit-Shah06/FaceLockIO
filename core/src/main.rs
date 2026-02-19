mod device_manager;
mod core_controller;
mod vision_bridge;
mod security_gate;
mod input_controller;

use std::io::{self, Write};

fn main() {
    println!("FSRC starting");
    core_controller::start();
    println!("FSRC exiting");
}
