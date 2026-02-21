use std::fs;
use std::path::Path;
use opencv::{
    prelude::*,
    videoio,
    imgcodecs,
    core,
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

    let save_path = format!("authorized_faces/{}.jpg",name);

    //-----------Initializing scanning here to capture face-----------
    {
        println!("Scaning.... Look at the camera.");

        let mut cam = videoio::VideoCapture::new(index, videoio::CAP_ANY).unwrap();
        let mut frame = Mat::default();
    
        thread::sleep(Duration::from_millis(500));

        if cam.read(&mut frame).unwrap() && !frame.empty() {
            let result = imgcodecs::imwrite(&save_path, &frame, &core::Vector::new());

            if result.is_ok() {
                println!("Successfully enroled: {}", name);
            }
            else {
                return false;
            }   
        }
        else {
            return false;
        }
    }
    true
}   


