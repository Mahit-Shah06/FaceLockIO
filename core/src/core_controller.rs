use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

pub fn start(){
    println!("Core Controller started");

    match crate::vision::start() {
        Ok(_) => {
            println!("Vision System Online.");
        },
        Err(e) => {
            println!("Stop; Error [{}] - {}", e.code(), e.message());
            return;
        }
    }
    crate::security_gate::start();
    crate::input_controller::start();

    loop {
        if crate::security_gate::SHOULD_STOP.load(Ordering::SeqCst) {
            println!("Core Controller: Stop Signal recieved from Security Gate.");
            break;
        }

        thread::sleep(Duration::from_millis(500));
    }

    stop();
}

pub fn stop(){
    crate::input_controller::stop();
    crate::vision::stop();
    crate::security_gate::stop();
    println!("Core Controller stopped");
}
