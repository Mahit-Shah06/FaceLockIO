pub mod camera_checker;
pub mod face_scanner;
pub mod face_recognizer;
pub mod face_manager;
use self::face_manager::FaceManager;
use crate::error::FsrcError;

pub fn start(enroll_mode: bool) -> Result<FaceManager, FsrcError> {
    println!("Vision Bridge Started");
    
    let (available, cam_list)   = camera_checker::is_cam_available();
    let mut manager = FaceManager::new()?;

    if !available {
        return Err(FsrcError::NoCameraFound);
    }

    let indice = cam_list[0].index;

    if !face_scanner::face_exist() {
        match face_scanner::enroll(indice, &manager) {
            Ok(true) => {
                manager.refresh_database()?;
            },
            Ok(false) => {
                if !face_scanner::face_exist(){
                    return Err(FsrcError::NoStoredFaces);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    
    manager.refresh_database()?;
    
    Ok(manager)
}

pub fn stop() {
    println!("Vision Bridge Stopped");
}
