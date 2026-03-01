use crate::vision::face_recognizer::FaceRecognizer;
use crate::error::FsrcError;
use opencv::prelude::*;

pub struct FaceManager {
    recognizer: FaceRecognizer,
    pub is_trained: bool,
}

impl FaceManager {
    pub fn new() -> Result<Self, FsrcError> {
        Ok(Self {
            recognizer: FaceRecognizer::new()?,
            is_trained: false,
        })
    }

    pub fn is_face_new(&self, frame: &Mat) -> bool {
        if !self.is_trained {
            return true;
        }

        match self.recognizer.predict(frame) {
            Ok(found) => !found,
            Err(_) => true,
        }

    }

    pub fn refresh_database(&mut self) -> Result<(), FsrcError> {
        self.recognizer.train_from_storage()?;
        self.is_trained = true;

        println!("FaceManager is now online");
        Ok(())
    }

    pub fn verify_identity(&self, frame: &Mat) -> bool {
        if !self.is_trained {
            return false;
        }

        self.recognizer.predict(frame).unwrap_or(false)
    }
}
