use sysinfo::{System, SystemExt, ProcessExt};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

pub fn get_status() {
    let mut sys = System::new_all();
    let pid = sysinfo::get_current_pid().unwrap();

    println!("Press Ctrl+C to exit Monitor Mode");

    loop {
        sys.refresh_all();
        
        // Clear terminal screen (ANSI escape code)
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        println!("=== FSRC REAL-TIME TELEMETRY ===");
        
        if let Some(process) = sys.process(pid) {
            let cpu = process.cpu_usage();
            let mem = process.memory() as f32 / 1024.0;
            
            println!("Process PID:  {}", pid);
            println!("CPU Usage:    {:.2}%", cpu);
            println!("Memory:       {:.2} MB", mem);
            
            // Visual progress bar for CPU
            let bars = (cpu / 10.0) as usize;
            println!("CPU Load:     [{}{}]", "|".repeat(bars), " ".repeat(10 - bars));
        }

        println!("-------------------------------");
        println!("System Uptime: {}s", sys.uptime());
        println!("Total RAM:     {} MB", sys.total_memory() / 1024);
        
        io::stdout().flush().unwrap();

        // Update every 1 second
        thread::sleep(Duration::from_secs(1));
    }
}
