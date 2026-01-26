mod core_controller;
mod vision_bridge;
mod security_gate;
mod input_controller;

fn main() {
    println!("FSRC starting");
    core_controller::start();
    core_controller::stop();
    println!("FSRC exiting");
}
