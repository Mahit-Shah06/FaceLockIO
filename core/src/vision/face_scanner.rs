use std::fs;
use std::path::Path;
use opencv::{
    prelude::*,
    videoio,
    imgcodecs,
    core,
    highgui,
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn face_exist() -> bool {
    let path = "authorized_faces";

    if !Path::new(path).exists() {
        let _ = fs::create_dir_all(path);
        return false;
    }

    if let Ok(entries) = fs::read_dir(path) {
        return entries.count() > 0;
    }

    false
}   

pub fn enroll(index: i32) -> bool {
    println!("-=-=-=Enrolling New Face=-=-=-");
    print!("Enter name for this face: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    println!("Opening Camera - Press 'S' to capture");

    let save_path = format!("authorized_faces/{}.jpg",name);

    let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY).unwrap();
    let mut frame = Mat::default();
    
    let window = "Scanning Face - Press 'S' to Capture";

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE).unwrap();

    loop {
        if cam.read(&mut frame).unwrap() && !frame.empty() {
            highgui::imshow(window, &frame).unwrap();
        }

        let key = highgui::wait_key(10).unwrap();
        if key == 115 {
            let result = imgcodecs::imwrite(&save_path, &frame, &core::Vector::new());
            if result.is_ok() {
                println!("Successfully enrolled: {}", name);
                break;
            }
        }

        if key == 27 {
            highgui::destroy_window(window).unwrap();
            return false;
        }
    }

    highgui::destroy_window(window).unwrap();
    true
}


