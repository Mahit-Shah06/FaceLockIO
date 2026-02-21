use opencv::{
    prelude::*,
    objdetect::CascadeClassifier,
    videoio,
    core,
    imgproc
};

pub fn detect_in_frame(frame: &core::Mat, detector: &mut CascadeClassifier) -> bool {
    if frame.empty() {return false; }

    let mut gray = core::Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unrwap();

    let mut faces = core::Vector::<core::React>::new();

    detector.detect_multi_scale(
        &gray, &mut faces, 1.1, 3, 0, 
        core::Size::new(30, 30), 
        core::Size::new(0,0)
    ).unwrap();

    faces.len() > 0
}
