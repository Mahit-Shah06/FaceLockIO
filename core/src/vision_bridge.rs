pub fn start() -> bool {

    println!("Vision Bridge Started");

    if is_cam_available(){
        println!("Camera available");
        true
    }
    else{
        println!("Camera unavailable");
        false
    }
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
