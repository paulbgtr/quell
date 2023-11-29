use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let sys = System::new_all();

    for (pid, process) in sys.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
}
