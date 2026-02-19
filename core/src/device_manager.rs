use evdev::enumerate;

pub struct InputDevice{
    pub name : String,
    pub path : std::path::PathBuf,
}

pub fn list_all_devices() -> Vec<InputDevice> {
    let mut devices = Vec::new();

    for (path, device) in enumerate() {
        devices.push(InputDevice {
            name: device.name().unwrap_or("Unknown Device").to_string(),
            path: path,
        });
    }
    devices
}

pub fn get_keyoards() -> Vec<InputDevice> {
    list_all_devices()
        .into_iter()
        .filter(|d| d.name.to_lowercase().contains("keyboard"))
        .collect()
}

pub fn get_pointers() -> Vec<InputDevice> {
    list_all_devices()
        .into_iter()
        .filter(|d| {
            let n = d.name.to_lowercase();
            n.contains("mouse") || n.contains("touchpad")
        })
        .collect()
}
