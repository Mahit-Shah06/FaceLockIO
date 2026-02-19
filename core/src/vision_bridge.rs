pub fn start() -> bool {
    println!("Vision bridge started");
    true
}

pub fn stop() {
    println!("Vision Bridge stopped");
}

fn is_cam_available() -> bool {
    true
}

fn is_face_present() -> bool{
    false
}
