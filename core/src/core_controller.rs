use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use crate::actions::FsrcAction;
use crate::vision;
use crate::security_gate;
use crate::input_controller;
use crate::system::telemetry;

pub fn start(action: FsrcAction){
    println!("Core Controller started; Mode [:?] enabled.", action);

    match action {
        FsrcAction::Monitor => {
            telemetry::get_status();
        }
        FsrcAction::ManageFaces => {
            let vis = vision::start(true);
        }
        FsrcAction::ManageDevices => {
            //let dev = input_controller::start(devices);
            println!("Device Module coming soon");
        }
        FsrcAction::ManageGesture => {
            //let ges = Gestures::start(menu);
            println!("Incoming gesture features");
        }
        FsrcAction::Onboard => {
            setup_wizard();
        }
        FsrcAction::Default => {
            run_system();
        }
        - => println!("Unknown action"),
    }
}

fn run_system() {
    match vision::start(false) {
        Ok(_) => println!("Vision System Online");
        Err(e) => {
            println!("An error occured : [{}] - {}", e.code(), e.message());
            return;
        }
    }

    security_gate::start();
    input_controller::start();

    loop {
        if crate::security_gate::SHOULD_STOP.load(Ordering::SeqCst) {
            println!("Core Controller: Stop SIgnal recieved");
            break;
        }
        
        //vision::is_face_present()

        thread::sleep(Duration::from_millis(100));
    }

    stop();
}

pub fn setup(){
    println!("Onboarding initialization");
}

pub fn stop(){
    crate::input_controller::stop();
    crate::vision::stop();
    crate::security_gate::stop();
    println!("Core Controller stopped");
}
