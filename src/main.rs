// use std::io::stdout;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut sys =
        System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
    sys.refresh_all();

    println!("SYSTEM INFO");
    print_system_information(std::time::Duration::from_secs(2), &mut sys);
}

fn print_system_information(duration: std::time::Duration, sys: &mut System) {
    loop {
        get_cpu_information(sys);
        get_memory_informaion(sys);
        std::thread::sleep(duration);
    }
}

fn get_cpu_information(sys: &mut System) {
    sys.refresh_cpu_usage();
    println!("CPU usage: {}%", sys.global_cpu_usage());
}

fn get_memory_informaion(sys: &mut System) {
    println!(
        "Memory usage: {} / {} GB",
        ((sys.used_memory() as f64 / 1e+6) / 1024.0) as u64,
        ((sys.total_memory() as f64 / 1e+6) / 1024.0) as u64
    );
}
