#[derive(Debug, Clone, Copy)]
pub enum FsrcError {
    NoCameraFound = 101,
    CameraAccessDenied = 102,
    CameraInuse = 103,

    ModelFileNotFound = 201,
    AuthorizedFolderMissing = 202,
    NoStoredFaces = 203,
    FaceAlreadyExist = 204,

    NotRunningAsRoot = 301,
    InputGrabFailed = 302,
}

impl FsrcError {
    /// Returns a human-readable message for the error code
    pub fn message(&self) -> &str {
        match self {
            FsrcError::FaceAlreadyExist => "This face is already registered in the system.",
            FsrcError::NoCameraFound => "No video devices detected in /dev/video*.",
            FsrcError::CameraAccessDenied => "Permission denied when accessing the camera.",
            FsrcError::ModelFileNotFound => "The Haar Cascade XML file was not found in /models.",
            FsrcError::NoStoredFaces => "No authorized faces found. Please run the scanner.",
            FsrcError::NotRunningAsRoot => "Root privileges (sudo) are required for this operation.",
            _ => "An unknown error occurred.",
        }
    }

    /// Returns the numeric code
    pub fn code(&self) -> i32 {
        // We cast the enum variant to its integer value
        *self as i32
    }
}
