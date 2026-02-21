pub mod camera_checker;
pub mod face_scanner;
pub mod face_recognizer;
use crate::errors::FsrcError;

pub fn start() -> Result<(), FsrcError> {
    println!("Vision Bridge Started");
    
    if !camera_checker::is_cam_available(){
        return Err(FsrcError::NoCameraFound);
    }

    if !face_scanner::faces_exist(){
        return Err(FsrcError::NoStoredFaces);
    }
}

pub fn stop() {
    println!("Vision Bridge Stopped");
}
