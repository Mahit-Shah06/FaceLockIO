pub fn start() {
    println!("Core Controller started");

    let cameraExists = crate::vision_bridge::start();
    if !cameraExists{
        println!("Camera does not exist");
        stop();
        return;
    }

    crate::security_gate::start();
    crate::input_controller::start();
}

pub fn stop() {
    crate::vision_bridge::stop();
    crate::security_gate::stop();
    crate::input_controller::stop();
    println!("Core Controller stopped");    
}
