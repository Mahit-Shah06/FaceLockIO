use evdev::{Device, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

pub fn start() {
    println!("Security Gate started");

    thread::spawn(|| {
        if let Err(e) = emergency_stop_listener() {
            eprintln!("Security Gate Error: {}", e);
        }
    });
}

pub fn stop() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
    println!("Security Gate stopped");
}

pub fn emergency_stop_listener() -> Result<(), Box<dyn std::error::Error>> {

    let device = Device::open("/dev/input/event2")?;
    println!("Security Gate: Monitoring keyboard ({})", device.name().unwrap_or("Unknown"));


    loop {
        if SHOULD_STOP.load(Ordering::SeqCst) { break; }

        let state = device.get_key_state()?;
        if state.contains(Key::KEY_LEFTCTRL) && state.contains(Key::KEY_LEFTALT) && 
            state.contains(Key::KEY_BACKSPACE) {
            println!("Emergency Stop");
            SHOULD_STOP.store(true, Ordering::SeqCst);
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
