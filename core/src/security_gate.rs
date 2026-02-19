use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub static SHOULD_STOP: AtomBool = AtomicBool::new(false);

pub fn start() {
    println!("Security Gate started");

    thread::spawn(|| {
        if let Err(e) = emergency_stop_listener() {
            eprintln!("Security Gate Error: {}", e);
        }
    )};
}

pub fn stop() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
    println!("Security Gate stopped");
}

pub fn emergency_stop_listener() -> Result<(), Box<dyn std::error::Error>> {

    println("Security Gate: Monitoring for emergency shortcut");

    loop {
        if SHOULD_STOP.load(Ordering::SeqCst){
            break;
        }

    thread::sleep(Duration::from_millis(100));
    }

    Ok(());
}
