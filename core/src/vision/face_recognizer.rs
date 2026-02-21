use opencv::{
    prelude::*,
    objdetect::CascadeClassifier,
    videoio,
    core,
    imgproc
};

pub struct FaceDetector {
    classifier: CascadeClassifier
}

impl FaceDetector {
    pub fn new(model_path: &str) -> Result<Self, opencv::Error> {
        let classifier = CascadeClassifier::new(model_path)?;

        Ok(Self { classifier })
    }
}

pub fn detect(&mut self, frame: &core::Mat) -> bool {
    let mut gray = core::Mat::default();
    imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    let mut faces = core::Vector::<core::Rect>::new();
    self.classifier.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        3,
        0, 30, 0
    ).unwrap();

    faces.len() > 0
}

