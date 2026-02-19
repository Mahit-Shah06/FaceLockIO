pub fn start() {
    println!("Core Controller started");

    crate::input_controller::start();
    crate::security_gate::start();
    loop{

        let faceDetected = crate::vision_bridge::check_for_face();

        if faceDetected{
            crate::input_controller:unblock();
        }else{
            crate::input_controller:block_input();
        }
        std::thread::sleep(std::time::Duration:from_secs(1));
    }
}

pub fn stop() {
    crate::vision_bridge::stop();
    crate::security_gate::stop();
    crate::input_controller::stop();
    println!("Core Controller stopped");    
}
