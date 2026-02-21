use std::fs;
use std::path::Path;

pub fn face_exist() -> bool {
    let path = "../authorized_faces";

    if !Path::new(path).exists() {
        let i = fs::create_dir_all(path);
        return false;
    }

    if let Ok(entries) = fs::read_dir(path) {
        return entries.count() > 0;
    }

    false
}   

pub fn scan(index: i32, save_path: &str) -> bool {
    println!("Scanning");
    
    let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default();
    
    if cam.read(&mut frame).unwrap() && !frame.empty() {
        let result = imgcodecs::imwrite(save_path, &frame, &core::Vector::new());
        return result.is_ok();
    }   
    false
}   

pub fn save() -> bool {
    true
}
