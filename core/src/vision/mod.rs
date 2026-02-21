pub mod camera_checker;

pub fn start() -> bool {
    println!("Vision Bridge Started");
    camera_checker::is_cam_available()
}

pub fn stop() {
    println!("Vision Bridge Stopped");
}
