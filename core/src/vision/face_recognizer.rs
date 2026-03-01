use opencv::{
    prelude::*,
    core,
    imgcodecs,
    imgproc,
    face::LBPHFaceRecognizer,
};
use std::fs;
use crate::error::FsrcError;

pub struct FaceRecognizer {
    model: core::Ptr<LBPHFaceRecognizer>,
}

impl FaceRecognizer {
    pub fn new() -> Result<Self, FsrcError> {
        let model = LBPHFaceRecognizer::create(1, 8, 8, 8, 75.0).map_err(|_| FsrcError::ModelFileNotFound)?;

        Ok(Self {model} )
    }

    pub fn train_from_storage(&mut self) -> Result<(), FsrcError> {
        let path = "authorized_faces";

        let mut images = core::Vector::<Mat>::new();
        let mut labels = core::Vector::<i32>::new();

        let entries = fs::read_dir(path).map_err(|_| FsrcError::AuthorizedFolderMissing)?;

        for (id, entry) in entries.enumerate() {
            let entry = entry.map_err(|_| FsrcError::NoStoredFaces)?;
            let img_path = entry.path().to_str().unwrap().to_string();

            let img = imgcodecs::imread(&img_path, imgcodecs::IMREAD_GRAYSCALE).map_err(|_| FsrcError::ModelFileNotFound)?;
            if !img.empty() {
                images.push(img);
                labels.push(0);
            }
        }
        self.model.train(&images, &labels).map_err(|_| FsrcError::ModelFileNotFound)?;

        Ok(())
    }

    pub fn predict(&self, frame: &Mat) -> Result<bool, FsrcError> {
        if frame.empty() {return Ok(false); }

        let mut gray  = Mat::default();
        imgproc::cvt_color(frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).map_err(|_| FsrcError::ModelFileNotFound)?;

        let mut label = -1;
        let mut confidence = 0.0;

        self.model.predict(&gray, &mut label, &mut confidence).map_err(|_| FsrcError::ModelFileNotFound)?;

        Ok(label == 0 && confidence < 70.0)
    }
}
