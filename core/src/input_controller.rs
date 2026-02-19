pub fn start() {
    println!("Input Controller started");
    
    let devices = crate::device_manager::list_all_devices();

    println!("------------------------------");
    if devices.is_empty() {
        println!("No devices found");
        return;
    }

    for dev in devices {
        println!("Device: {} | Path:{:?}", dev.name, dev.path);
    }
    println!("------------------------------");
}

pub fn block_inputs() {
    println!("Blocking selected inputs");
}

pub fn unblock(){
    println!("Selected inputs unblocked");
}

pub fn stop() {
    unblock();
    println!("Input Controller stopped");
}
