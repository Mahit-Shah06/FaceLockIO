pub fn start() {
    println!("Input Controller started");
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
