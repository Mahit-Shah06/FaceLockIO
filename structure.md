FaceLockIO/
├── main.rs                 # Entry point: Orchestrates the modules
├── src/                    # Logic folder
│   ├── vision_bridge.rs    # Face detection (OpenCV/Haar Cascades)
│   ├── input_manager.rs    # evdev logic to grab/release hardware
│   ├── security_gate.rs    # Shortcut key & Emergency kill-switch
│   ├── db_connector.rs     # SQLite/File-based storage for faces
│   └── config_loader.rs    # User settings for which USBs to block
└── Cargo.toml              # Dependencies
