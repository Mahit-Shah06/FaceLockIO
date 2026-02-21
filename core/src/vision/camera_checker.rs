use std::fs;
use std::path::Path;

pub struct CameraInfo{
    pub index: i32,
    pub name: String,
}

pub fn is_cam_available() -> bool {
    let available_cameras = list_available_cameras();
    if !available_cameras.is_empty() {
        println!("Vision: Found {} working camera(s):", available_cameras.len());
        for cam in &available_cameras {
            println!("  -> [{}] {}", cam.index, cam.name);
        }
        true
    } else {
        println!("Vision: No cameras detected.");
        false
    }
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
