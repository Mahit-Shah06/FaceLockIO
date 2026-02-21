pub mod camera_checker;
pub mod face_scanner;
//pub mod face_recognizer;
use crate::error::FsrcError;

pub fn start() -> Result<(), FsrcError> {
    println!("Vision Bridge Started");
    
    let (available, cam_list) = camera_checker::is_cam_available();

    if !available {
        return Err(FsrcError::NoCameraFound);
    }

    if !face_scanner::face_exist(){
        let indice = cam_list[0].index;
        if !face_scanner::enroll(indice) {
            return Err(FsrcError::NoStoredFaces);
        }
    }

    Ok(())
}

pub fn stop() {
    println!("Vision Bridge Stopped");
}
