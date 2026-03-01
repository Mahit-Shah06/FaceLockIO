#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsrcAction{
    Onboard = 0,
    Monitor = 1,
    ManageFaces = 2,
    ManageDevices = 3,
    ManageGestures = 4,
}

impl FsrcAction{
    pub fn from_u32(values: u32) -> Self {
        match value {
            1 => FsrcAction::Monitor,
            2 => FsrcAction::ManageFaces,
            3 => FsrcAction::ManageDevices,
            4 => FsrcAction::ManageGestures,
            _ => FsrcAction::Onboard,
        }
    }
}
