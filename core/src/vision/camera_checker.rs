use std::fs;
use std::path::Path;

pub struct CameraInfo{
    pub index: i32,
    pub name: String,
}

pub fn is_cam_available() -> (bool, Vec<CameraInfo>) {
    let available_cameras = list_available_cameras();
    let success =  !available_cameras.is_empty();

    return (success, available_cameras)
}

fn list_available_cameras() -> Vec<CameraInfo> {
    let mut cams = Vec::new();

    for i in 0..5{
        let device_path = format!("/dev/video{}", i);

        if Path::new(&device_path).exists() {
            let name_path = format!("/sys/class/video4linux/video{}/name", i);
            let name = fs::read_to_string(name_path)
                .unwrap_or_else(|_| format!("Unknown camera {}", i))
                .trim()
                .to_string();
            if !name.contains("Metadata"){
                cams.push(CameraInfo{
                    index: i,
                    name,
                });
            }
        }
    }
    cams
}
