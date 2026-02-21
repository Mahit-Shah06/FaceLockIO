use opencv::{
    prelude::*,
    objdetect::CascadeClassifier,
    videoio,
    core,
    imgproc,
};

pub fn start() -> bool {
    println!("Vision bridge started");

    if !is_cam_available(){
        println!("Error: No cameras detected on this device.");
        return false;
    }
    true

}

fn is_cam_available() -> bool {
    let available_cameras = list_available_cameras();
    return !available_cameras.is_empty();
}

fn list_available_cameras() -> Vec<i32> {
    let mut cams = Vec::new();

    for i in 0..5{
        if let Ok(cam) = videoio::VideoCapture::new(i, videoio::CAP_ANY) {
            if let Ok(opened) = videoio::VideoCapture::is_opened(&cam) {
                if opened{
                    cams.push(i);
                }
            }
        }
    }
    cams
}

fn is_face_present() -> bool{
    false
}

pub fn stop() {
    println!("Vision Bridge stopped");
}

